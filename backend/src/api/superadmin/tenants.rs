use crate::{
    core::rbac::middleware::require_super_admin,
    infrastructure::database::tenants::TenantRepository,
    models::tenant::Tenant,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post, put},
    Json, Router, Extension,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Serialize)]
pub struct TenantListItem {
    pub id: Uuid,
    pub business_name: String,
    pub cuit: String,
    pub phone: Option<String>,
    pub status: Option<String>,
    pub is_active: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct CreateTenantDto {
    pub business_name: String,
    pub cuit: String,
    pub dni_responsable: String,
    pub first_name: String,
    pub last_name: String,
    pub admin_email: String,
    pub phone: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateTenantStatusDto {
    pub status: String, // 'ACTIVE', 'PENDING', 'SUSPENDED', 'DELETED'
}

async fn list_tenants(
    State(pool): State<Arc<PgPool>>,
) -> Result<Json<Vec<TenantListItem>>, StatusCode> {
    let tenants = sqlx::query_as!(
        TenantListItem,
        r#"
        SELECT id, business_name, cuit, phone, status, is_active, created_at
        FROM tenants
        WHERE status != 'DELETED' OR status IS NULL
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| {
        tracing::error!("Error listing tenants: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(tenants))
}

async fn get_tenant(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Tenant>, StatusCode> {
    let repo = TenantRepository::new(pool);
    let tenant = repo
        .find_by_id_any_status(id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(tenant))
}

async fn create_tenant(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<crate::core::security::jwt::Claims>,
    Json(payload): Json<CreateTenantDto>,
) -> Result<Json<Tenant>, axum::response::Response> {
    let mut slug = payload.business_name.to_lowercase().replace(" ", "-");
    slug.retain(|c| c.is_ascii_alphanumeric() || c == '-');
    
    let mut tx = pool.begin().await.map_err(|e| {
        tracing::error!("Failed to begin transaction: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(serde_json::json!({"error": "Error interno"}))).into_response()
    })?;

    let tenant = match sqlx::query_as::<_, Tenant>(
        r#"
        INSERT INTO tenants (business_name, cuit, dni_responsable, first_name, last_name, phone, status, slug, is_active)
        VALUES ($1, $2, $3, $4, $5, $6, 'ACTIVE', $7, true)
        RETURNING *
        "#
    )
    .bind(&payload.business_name)
    .bind(&payload.cuit)
    .bind(&payload.dni_responsable)
    .bind(&payload.first_name)
    .bind(&payload.last_name)
    .bind(&payload.phone)
    .bind(slug)
    .fetch_one(&mut *tx)
    .await
    {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("Error creating tenant: {}", e);
            let _ = tx.rollback().await;
            if let Some(db_err) = e.as_database_error() {
                let msg = db_err.message();
                if msg.contains("tenants_cuit_key") || msg.contains("cuit") {
                    let err_resp = (StatusCode::CONFLICT, axum::Json(serde_json::json!({"error": "El CUIT ingresado ya está registrado"}))).into_response();
                    return Err(err_resp);
                } else if msg.contains("tenants_slug_key") || msg.contains("slug") {
                    let err_resp = (StatusCode::CONFLICT, axum::Json(serde_json::json!({"error": "El nombre de inmobiliaria genera un identificador ya en uso"}))).into_response();
                    return Err(err_resp);
                }
            }
            let err_resp = (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(serde_json::json!({"error": "Error interno al crear inmobiliaria"}))).into_response();
            return Err(err_resp);
        }
    };

    let trial_ends_at = Utc::now() + chrono::Duration::days(14);
    
    if let Err(e) = sqlx::query(
        r#"INSERT INTO subscriptions (tenant_id, plan_type, status, trial_ends_at)
           VALUES ($1, 'BASIC'::plan_type, 'TRIAL'::subscription_status, $2)"#,
    )
    .bind(tenant.id)
    .bind(trial_ends_at)
    .execute(&mut *tx)
    .await
    {
        tracing::error!("Error creating subscription: {}", e);
        let _ = tx.rollback().await;
        return Err((StatusCode::INTERNAL_SERVER_ERROR, axum::Json(serde_json::json!({"error": "Error interno"}))).into_response());
    }

    let user_id = Uuid::new_v4();
    let onboarding_token = Uuid::new_v4().to_string();
    let onboarding_expires = Utc::now() + chrono::Duration::days(7);

    if let Err(e) = sqlx::query(
        r#"INSERT INTO users (id, tenant_id, role, email, password_hash, first_name, last_name, is_active, onboarding_token, onboarding_token_expires_at)
           VALUES ($1, $2, 'ADMIN_INMOBILIARIA'::user_role, $3, '', $4, $5, true, $6, $7)"#,
    )
    .bind(user_id)
    .bind(tenant.id)
    .bind(&payload.admin_email)
    .bind(&payload.first_name)
    .bind(&payload.last_name)
    .bind(&onboarding_token)
    .bind(onboarding_expires)
    .execute(&mut *tx)
    .await
    {
        tracing::error!("Error creating user: {}", e);
        let _ = tx.rollback().await;
        if e.to_string().contains("users_email_key") {
            return Err((StatusCode::CONFLICT, axum::Json(serde_json::json!({"error": "El correo electrónico ya está en uso"}))).into_response());
        }
        return Err((StatusCode::INTERNAL_SERVER_ERROR, axum::Json(serde_json::json!({"error": "Error al crear administrador"}))).into_response());
    }

    let audit_user_id = if claims.sub == Uuid::nil() { None } else { Some(claims.sub) };
    
    if let Err(e) = sqlx::query(
        r#"INSERT INTO audit_logs (tenant_id, user_id, action, new_data)
           VALUES ($1, $2, 'TENANT_CREATED_BY_SUPERADMIN', $3)"#,
    )
    .bind(tenant.id)
    .bind(audit_user_id)
    .bind(serde_json::json!({ "business_name": payload.business_name, "cuit": payload.cuit }))
    .execute(&mut *tx)
    .await
    {
        tracing::error!("Error creating audit log: {}", e);
        // non-fatal
    }

    if let Err(e) = tx.commit().await {
        tracing::error!("Error committing transaction: {}", e);
        return Err((StatusCode::INTERNAL_SERVER_ERROR, axum::Json(serde_json::json!({"error": "Error interno"}))).into_response());
    }

    let frontend_url = std::env::var("FRONTEND_URL")
        .unwrap_or_else(|_| "https://inmonea.agentech.ar".to_string());
    let magic_link = format!("{}/onboarding?token={}", frontend_url, onboarding_token);
    tracing::info!("ONBOARDING EMAIL SIMULADO: Enviar a {} link: {}", payload.admin_email, magic_link);

    // Send WhatsApp if phone is available
    if let Some(phone) = &payload.phone {
        if !phone.trim().is_empty() {
            let msg = format!("¡Hola {}! Tu inmobiliaria {} ha sido dada de alta exitosamente en nuestra plataforma SaaS.\n\nPara acceder por primera vez y configurar tu contraseña, ingresa al siguiente enlace (valido por 7 días):\n{}", payload.first_name, payload.business_name, magic_link);
            let p = phone.clone();
            tokio::spawn(async move {
                let evo_client = crate::infrastructure::evolution::client::EvolutionClient::new();
                match evo_client.send_message(&p, &msg).await {
                    Ok(_) => tracing::info!("WhatsApp de onboarding enviado a {}", p),
                    Err(e) => tracing::error!("Error enviando WhatsApp de onboarding a {}: {}", p, e),
                }
            });
        }
    }

    Ok(Json(tenant))
}

async fn update_tenant_status(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTenantStatusDto>,
) -> Result<StatusCode, StatusCode> {
    let is_active = payload.status == "ACTIVE";

    let result = sqlx::query!(
        "UPDATE tenants SET status = $1, is_active = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3",
        payload.status,
        is_active,
        id
    )
    .execute(&*pool)
    .await
    .map_err(|e| {
        tracing::error!("Error updating tenant status: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    tracing::info!("TENANT_STATUS_UPDATED: tenant_id={} new_status={}", id, payload.status);

    Ok(StatusCode::NO_CONTENT)
}

pub fn router(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/", get(list_tenants).post(create_tenant))
        .route("/:id", get(get_tenant))
        .route("/:id/status", put(update_tenant_status))
        .route_layer(middleware::from_fn(require_super_admin))
        .route_layer(middleware::from_fn_with_state(pool.clone(), crate::core::tenant::middleware::tenant_middleware))
        .with_state(pool)
}
