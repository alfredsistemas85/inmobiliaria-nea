use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Extension, Json,
};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    api::leads::dtos::{CreateLeadDto, UpdateLeadDto},
    core::security::jwt::Claims,
    infrastructure::database::{
        audit::AuditRepository, leads::LeadRepository, notifications::NotificationRepository,
    },
    models::{
        common::{PaginatedResponse, PaginationParams},
        lead::Lead,
    },
};

pub async fn list_leads(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<Lead>>, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::FORBIDDEN)?;
    let repo = LeadRepository::new(pool);

    let limit = params.limit.unwrap_or(50);
    let offset = params.offset.unwrap_or(0);
    let q = params.q.as_deref();

    let leads = repo
        .list(tenant_id, limit, offset, q)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(leads))
}

pub async fn get_lead(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Lead>, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::FORBIDDEN)?;
    let repo = LeadRepository::new(pool);

    let lead = repo
        .get_by_id(id, tenant_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(lead))
}

pub async fn create_lead(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateLeadDto>,
) -> Result<Json<Lead>, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::FORBIDDEN)?;
    let repo = LeadRepository::new(pool.clone());
    let audit_repo = AuditRepository::new(pool.clone());

    let status = payload.status.unwrap_or_else(|| "NUEVO".to_string());
    let source = payload.source.unwrap_or_else(|| "MANUAL".to_string());

    let lead = repo
        .create(
            tenant_id,
            payload.client_id,
            payload.property_id,
            Some(&status),
            Some(&source),
            payload.assigned_to,
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let _ = audit_repo
        .log(
            Some(tenant_id),
            Some(claims.sub),
            "CREATE_LEAD",
            "lead",
            Some(lead.id),
            None,
        )
        .await;

    let _ = repo
        .log_activity(
            tenant_id,
            lead.id,
            Some(claims.sub),
            "StatusChange",
            Some(&format!("Lead creado en estado {}", status)),
        )
        .await;

    tracing::info!(
        "LEAD_CREATED: tenant_id={} lead_id={} client_id={} property_id={:?}",
        tenant_id,
        lead.id,
        payload.client_id,
        payload.property_id
    );

    Ok(Json(lead))
}

pub async fn update_lead(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<UpdateLeadDto>,
) -> Result<Json<Lead>, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::FORBIDDEN)?;
    let repo = LeadRepository::new(pool.clone());
    let audit_repo = AuditRepository::new(pool.clone());

    let old_lead = repo
        .get_by_id(id, tenant_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let lead = repo
        .update(
            id,
            tenant_id,
            payload.status.as_deref(),
            payload.assigned_to,
            payload.source.as_deref(),
        )
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    if let Some(new_status) = payload.status {
        if old_lead.status.as_deref() != Some(&new_status) {
            let _ = repo
                .log_activity(
                    tenant_id,
                    lead.id,
                    Some(claims.sub),
                    "StatusChange",
                    Some(&format!("Estado cambiado a {}", new_status)),
                )
                .await;
            tracing::info!(
                "LEAD_STATUS_CHANGED: tenant_id={} lead_id={} new_status={}",
                tenant_id,
                lead.id,
                new_status
            );
        }
    }

    if payload.assigned_to.is_some() && old_lead.assigned_to != payload.assigned_to {
        let notif_repo = NotificationRepository::new(pool.clone());
        let _ = notif_repo
            .create(
                tenant_id,
                payload.assigned_to,
                "LEAD_ASSIGNED",
                "Lead Asignado",
                "Se te ha asignado un nuevo lead",
            )
            .await;
    }

    let _ = audit_repo
        .log(
            Some(tenant_id),
            Some(claims.sub),
            "UPDATE_LEAD",
            "lead",
            Some(lead.id),
            None,
        )
        .await;

    tracing::info!("LEAD_UPDATED: tenant_id={} lead_id={}", tenant_id, lead.id);

    Ok(Json(lead))
}

pub async fn convert_lead(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Lead>, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::FORBIDDEN)?;
    let repo = LeadRepository::new(pool.clone());
    let audit_repo = AuditRepository::new(pool.clone());

    let lead = repo
        .update(id, tenant_id, Some("CERRADO_GANADO"), None, None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let _ = repo
        .log_activity(
            tenant_id,
            lead.id,
            Some(claims.sub),
            "Conversion",
            Some("Lead convertido a Cliente Exitosamente"),
        )
        .await;

    let _ = audit_repo
        .log(
            Some(tenant_id),
            Some(claims.sub),
            "CONVERT_LEAD",
            "lead",
            Some(lead.id),
            None,
        )
        .await;

    tracing::info!(
        "LEAD_STATUS_CHANGED: tenant_id={} lead_id={} new_status=CERRADO_GANADO",
        tenant_id,
        lead.id
    );

    Ok(Json(lead))
}

pub async fn delete_lead(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> Result<StatusCode, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::FORBIDDEN)?;
    let repo = LeadRepository::new(pool.clone());
    let audit_repo = AuditRepository::new(pool);

    let rows_affected = repo
        .soft_delete(id, tenant_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if rows_affected > 0 {
        let _ = audit_repo
            .log(
                Some(tenant_id),
                Some(claims.sub),
                "DELETE_LEAD",
                "lead",
                Some(id),
                None,
            )
            .await;

        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
