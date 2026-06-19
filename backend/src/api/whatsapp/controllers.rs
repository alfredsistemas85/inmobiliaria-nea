use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Extension, Json,
};
use serde::Deserialize;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    api::whatsapp::dtos::{AssignConversationDto, WebhookPayload},
    core::security::jwt::Claims,
    infrastructure::database::{
        audit::AuditRepository, clients::ClientRepository, leads::LeadRepository,
        notifications::NotificationRepository, whatsapp::WhatsAppRepository,
    },
    infrastructure::evolution::client::EvolutionClient,
    models::common::PaginatedResponse,
    models::whatsapp::{ConversationListItem, Message},
};

#[derive(Deserialize)]
pub struct SendMessageDto {
    pub content: String,
}

#[derive(Deserialize)]
pub struct PaginationQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

pub async fn list_conversations(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<PaginatedResponse<ConversationListItem>>, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::FORBIDDEN)?;
    let limit = query.limit.unwrap_or(50);
    let offset = (query.page.unwrap_or(1) - 1) * limit;

    let repo = WhatsAppRepository::new(pool);
    let result = repo
        .list_conversations(tenant_id, limit, offset)
        .await
        .map_err(|e| {
            tracing::error!("Error listing conversations: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(result))
}

pub async fn list_messages(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
    Path(conversation_id): Path<Uuid>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<PaginatedResponse<Message>>, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::FORBIDDEN)?;
    let limit = query.limit.unwrap_or(50);
    let offset = (query.page.unwrap_or(1) - 1) * limit;

    let repo = WhatsAppRepository::new(pool.clone());

    // Check if conversation belongs to tenant
    // It's verified implicitly by tenant_id in list_messages

    // Reset unread count when fetching messages
    let _ = repo.reset_unread_count(tenant_id, conversation_id).await;

    let result = repo
        .list_messages(tenant_id, conversation_id, limit, offset)
        .await
        .map_err(|e| {
            tracing::error!("Error listing messages: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(result))
}

pub async fn send_chat_message(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
    Path(conversation_id): Path<Uuid>,
    Json(payload): Json<SendMessageDto>,
) -> Result<Json<Message>, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::FORBIDDEN)?;
    let user_id = claims.sub;

    // Get conversation to find client's phone
    let conv = sqlx::query!(
        "SELECT c.id, cl.phone FROM conversations c JOIN clients cl ON c.client_id = cl.id WHERE c.id = $1 AND c.tenant_id = $2 AND c.deleted_at IS NULL AND cl.deleted_at IS NULL",
        conversation_id, tenant_id
    )
    .fetch_optional(&*pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    let evo_client = EvolutionClient::new();

    // We send message
    evo_client
        .send_message(&conv.phone, &payload.content)
        .await
        .map_err(|e| {
            tracing::error!("Error sending manual WA message: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Insert to DB
    let repo = WhatsAppRepository::new(pool.clone());
    let msg = repo
        .insert_message(
            tenant_id,
            conversation_id,
            None, // outbound messages don't have an immediate external_id usually unless we parse response
            "outbound",
            "agent",
            &payload.content,
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .unwrap(); // Cannot be None since external_id is None

    // Audit Log
    let audit_repo = AuditRepository::new(pool);
    let _ = audit_repo
        .log(
            Some(tenant_id),
            Some(user_id),
            "MESSAGE_SENT",
            "Message",
            Some(msg.id),
            None,
        )
        .await;

    Ok(Json(msg))
}

pub async fn webhook(
    State(pool): State<Arc<PgPool>>,
    Path(tenant_id): Path<Uuid>,
    Json(payload): Json<WebhookPayload>,
) -> Result<StatusCode, StatusCode> {
    if let Some(event) = &payload.event {
        if event == "messages.upsert" {
            if let Some(data) = payload.data {
                if let Some(messages) = data.get("messages").and_then(|m| m.as_array()) {
                    let client_repo = ClientRepository::new(pool.clone());
                    let lead_repo = LeadRepository::new(pool.clone());
                    let wa_repo = WhatsAppRepository::new(pool.clone());
                    let audit_repo = AuditRepository::new(pool.clone());
                    let notif_repo = NotificationRepository::new(pool.clone());

                    for msg in messages {
                        let from_me = msg
                            .pointer("/key/fromMe")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(true);
                        if from_me {
                            continue;
                        }

                        let remote_jid = msg
                            .pointer("/key/remoteJid")
                            .and_then(|v| v.as_str())
                            .unwrap_or("");
                        let external_id = msg
                            .pointer("/key/id")
                            .and_then(|v| v.as_str())
                            .unwrap_or("");
                        let phone = remote_jid.split('@').next().unwrap_or("").to_string();

                        // Extract text
                        let content = msg
                            .pointer("/message/conversation")
                            .or_else(|| msg.pointer("/message/extendedTextMessage/text"))
                            .and_then(|v| v.as_str())
                            .unwrap_or("");

                        if phone.is_empty() || phone.contains("g.us") || content.is_empty() {
                            continue;
                        }

                        // Check or create client
                        let mut client_id = Uuid::nil();
                        let existing_client = client_repo
                            .list(tenant_id, 1, 0, Some(&phone))
                            .await
                            .ok()
                            .and_then(|res| res.data.into_iter().next());

                        if let Some(client) = existing_client {
                            client_id = client.id;
                        } else {
                            if let Ok(new_client) = client_repo
                                .create(tenant_id, Some("Nuevo Contacto"), None, &phone, None, None)
                                .await
                            {
                                client_id = new_client.id;
                                let _ = lead_repo
                                    .create(
                                        tenant_id,
                                        client_id,
                                        None,
                                        Some("NUEVO"),
                                        Some("WhatsApp Automático"),
                                        None,
                                    )
                                    .await;
                            }
                        }

                        if client_id != Uuid::nil() {
                            // Find or create conversation
                            if let Ok(conv) = wa_repo
                                .find_or_create_conversation(tenant_id, client_id)
                                .await
                            {
                                // Insert message
                                if let Ok(Some(inserted_msg)) = wa_repo
                                    .insert_message(
                                        tenant_id,
                                        conv.id,
                                        Some(external_id.to_string()),
                                        "inbound",
                                        "client",
                                        content,
                                    )
                                    .await
                                {
                                    let _ = audit_repo
                                        .log(
                                            Some(tenant_id),
                                            None,
                                            "MESSAGE_RECEIVED",
                                            "Message",
                                            Some(inserted_msg.id),
                                            None,
                                        )
                                        .await;

                                    // Send notification
                                    let _ = notif_repo
                                        .create(
                                            tenant_id,
                                            conv.assigned_user_id,
                                            "NEW_MESSAGE",
                                            "Nuevo Mensaje de WhatsApp",
                                            &format!("Has recibido un nuevo mensaje de {}", phone),
                                        )
                                        .await;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(StatusCode::OK)
}

pub async fn run_reminders(State(pool): State<Arc<PgPool>>) -> Result<StatusCode, StatusCode> {
    // Same as Phase 2
    let rows = sqlx::query!(
        r#"
        SELECT a.id, a.tenant_id, a.scheduled_at, c.id as client_id, c.phone, c.first_name, p.title as property_title
        FROM appointments a
        JOIN clients c ON a.client_id = c.id
        LEFT JOIN properties p ON a.property_id = p.id
        WHERE a.scheduled_at > CURRENT_TIMESTAMP 
        AND a.scheduled_at <= CURRENT_TIMESTAMP + INTERVAL '24 hours'
        AND a.confirmation_sent_at IS NULL
        AND a.deleted_at IS NULL
        "#
    )
    .fetch_all(&*pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let evo_client = EvolutionClient::new();
    let wa_repo = WhatsAppRepository::new(pool.clone());

    for row in rows {
        let phone = row.phone;
        let name = row.first_name.unwrap_or_else(|| "Cliente".to_string());
        let property = row
            .property_title
            .unwrap_or_else(|| "la propiedad".to_string());
        let time = row.scheduled_at.format("%Y-%m-%d %H:%M").to_string();

        let message = format!(
            "Hola {}, te recordamos tu cita para {} el {}. Por favor confirmar.",
            name, property, time
        );

        if evo_client.send_message(&phone, &message).await.is_ok() {
            let _ = sqlx::query!(
                "UPDATE appointments SET confirmation_sent_at = CURRENT_TIMESTAMP WHERE id = $1",
                row.id
            )
            .execute(&*pool)
            .await;

            // Insert into history
            if let Ok(conv) = wa_repo
                .find_or_create_conversation(row.tenant_id, row.client_id)
                .await
            {
                let _ = wa_repo
                    .insert_message(row.tenant_id, conv.id, None, "outbound", "bot", &message)
                    .await;
            }
        }
    }

    Ok(StatusCode::OK)
}

pub async fn take_conversation(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::FORBIDDEN)?;
    let user_id = claims.sub;

    let repo = WhatsAppRepository::new(pool.clone());
    repo.update_assignment(tenant_id, id, Some(user_id))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let audit = AuditRepository::new(pool);
    let _ = audit
        .log(
            Some(tenant_id),
            Some(user_id),
            "CONVERSATION_TAKEN",
            "conversations",
            Some(id),
            None,
        )
        .await;

    Ok(StatusCode::OK)
}

pub async fn assign_conversation(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
    Json(payload): Json<AssignConversationDto>,
) -> Result<StatusCode, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::FORBIDDEN)?;
    let user_id = claims.sub;

    let repo = WhatsAppRepository::new(pool.clone());
    repo.update_assignment(tenant_id, id, Some(payload.user_id))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let audit = AuditRepository::new(pool);
    let _ = audit
        .log(
            Some(tenant_id),
            Some(user_id),
            "CONVERSATION_ASSIGNED",
            "conversations",
            Some(id),
            Some(serde_json::json!({"assigned_to": payload.user_id})),
        )
        .await;

    Ok(StatusCode::OK)
}

pub async fn unassign_conversation(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::FORBIDDEN)?;
    let user_id = claims.sub;

    let repo = WhatsAppRepository::new(pool.clone());
    repo.update_assignment(tenant_id, id, None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let audit = AuditRepository::new(pool);
    let _ = audit
        .log(
            Some(tenant_id),
            Some(user_id),
            "CONVERSATION_UNASSIGNED",
            "conversations",
            Some(id),
            None,
        )
        .await;

    Ok(StatusCode::OK)
}

pub async fn close_conversation(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::FORBIDDEN)?;
    let user_id = claims.sub;

    let repo = WhatsAppRepository::new(pool.clone());
    repo.update_status(tenant_id, id, "CLOSED")
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let audit = AuditRepository::new(pool);
    let _ = audit
        .log(
            Some(tenant_id),
            Some(user_id),
            "CONVERSATION_CLOSED",
            "conversations",
            Some(id),
            None,
        )
        .await;

    Ok(StatusCode::OK)
}

pub async fn reopen_conversation(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::FORBIDDEN)?;
    let user_id = claims.sub;

    let repo = WhatsAppRepository::new(pool.clone());
    repo.update_status(tenant_id, id, "OPEN")
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let audit = AuditRepository::new(pool);
    let _ = audit
        .log(
            Some(tenant_id),
            Some(user_id),
            "CONVERSATION_REOPENED",
            "conversations",
            Some(id),
            None,
        )
        .await;

    Ok(StatusCode::OK)
}

#[derive(Deserialize)]
pub struct CreateInstanceDto {
    pub instance_name: String,
}

pub async fn get_instance_status(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::FORBIDDEN)?;
    let repo = WhatsAppRepository::new(pool.clone());

    let db_instance = repo
        .get_instance(tenant_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(mut inst) = db_instance {
        if let Some(name) = inst.get("instance_name").and_then(|v| v.as_str()) {
            let evo = EvolutionClient::new();
            if let Ok(state) = evo.get_instance_state(name).await {
                let current_state = state
                    .pointer("/instance/state")
                    .and_then(|v| v.as_str())
                    .unwrap_or("DISCONNECTED");
                if inst.get("status").and_then(|v| v.as_str()) != Some(current_state) {
                    let _ = repo
                        .upsert_instance(tenant_id, name, current_state, None, None)
                        .await;
                    inst["status"] = serde_json::json!(current_state);
                }
            }
        }
        Ok(Json(inst))
    } else {
        Ok(Json(serde_json::json!(null)))
    }
}

pub async fn create_instance(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateInstanceDto>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::FORBIDDEN)?;

    let evo = EvolutionClient::new();
    let res = evo
        .create_instance(&payload.instance_name)
        .await
        .map_err(|e| {
            tracing::error!("Error creating Evolution instance: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let qr = res.pointer("/qrcode/base64").and_then(|v| v.as_str());
    let repo = WhatsAppRepository::new(pool);
    let _ = repo
        .upsert_instance(tenant_id, &payload.instance_name, "CREATED", qr, None)
        .await;

    tracing::info!(
        "INSTANCE_CREATED: tenant_id={} instance_name={}",
        tenant_id,
        payload.instance_name
    );

    Ok(Json(res))
}

pub async fn get_qr(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::FORBIDDEN)?;
    let repo = WhatsAppRepository::new(pool.clone());

    let db_instance = repo
        .get_instance(tenant_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let name = db_instance
        .get("instance_name")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let evo = EvolutionClient::new();
    let res = evo
        .connect_instance(name)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let qr = res
        .pointer("/qrcode")
        .or_else(|| res.pointer("/base64"))
        .and_then(|v| v.as_str());
    if let Some(q) = qr {
        let _ = repo
            .upsert_instance(tenant_id, name, "CONNECTING", Some(q), None)
            .await;
        tracing::info!(
            "QR_GENERATED: tenant_id={} instance_name={}",
            tenant_id,
            name
        );
    } else {
        // Si Evolution no devuelve QR, podría significar que ya está conectada.
        // Lo trataremos como estado de conexión.
        tracing::info!(
            "INSTANCE_CONNECTED: tenant_id={} instance_name={}",
            tenant_id,
            name
        );
    }

    Ok(Json(res))
}

pub async fn logout_instance(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
) -> Result<StatusCode, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::FORBIDDEN)?;
    let repo = WhatsAppRepository::new(pool.clone());

    let db_instance = repo
        .get_instance(tenant_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let name = db_instance
        .get("instance_name")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let evo = EvolutionClient::new();
    let _ = evo.logout_instance(name).await; // ignore error

    let _ = repo
        .upsert_instance(tenant_id, name, "DISCONNECTED", None, None)
        .await;

    tracing::warn!(
        "INSTANCE_DISCONNECTED: tenant_id={} instance_name={}",
        tenant_id,
        name
    );

    Ok(StatusCode::OK)
}
