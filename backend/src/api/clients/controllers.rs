use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Extension, Json,
};
use serde_json::json;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    api::clients::dtos::{CreateClientDto, UpdateClientDto},
    core::security::jwt::Claims,
    infrastructure::database::{audit::AuditRepository, clients::ClientRepository},
    models::{
        client::Client,
        common::{PaginatedResponse, PaginationParams},
    },
};

pub async fn list_clients(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<Client>>, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::FORBIDDEN)?;
    let repo = ClientRepository::new(pool);

    let limit = params.limit.unwrap_or(20);
    let offset = params.offset.unwrap_or(0);
    let q = params.q.as_deref();

    let clients = repo
        .list(tenant_id, limit, offset, q)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(clients))
}

pub async fn get_client(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Client>, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::FORBIDDEN)?;
    let repo = ClientRepository::new(pool);

    let client = repo
        .get_by_id(id, tenant_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(client))
}

pub async fn create_client(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateClientDto>,
) -> Result<Json<Client>, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::FORBIDDEN)?;
    let repo = ClientRepository::new(pool.clone());
    let audit_repo = AuditRepository::new(pool);

    let client = repo
        .create(
            tenant_id,
            payload.first_name.as_deref(),
            payload.last_name.as_deref(),
            &payload.phone,
            payload.email.as_deref(),
            payload.notes.as_deref(),
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Log audit
    let _ = audit_repo
        .log(
            Some(tenant_id),
            Some(claims.sub),
            "CREATE_CLIENT",
            "client",
            Some(client.id),
            Some(json!({ "phone": client.phone })),
        )
        .await;

    Ok(Json(client))
}

pub async fn update_client(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<UpdateClientDto>,
) -> Result<Json<Client>, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::FORBIDDEN)?;
    let repo = ClientRepository::new(pool.clone());
    let audit_repo = AuditRepository::new(pool);

    let client = repo
        .update(
            id,
            tenant_id,
            payload.first_name.as_deref(),
            payload.last_name.as_deref(),
            payload.phone.as_deref(),
            payload.email.as_deref(),
            payload.notes.as_deref(),
        )
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    // Log audit
    let _ = audit_repo
        .log(
            Some(tenant_id),
            Some(claims.sub),
            "UPDATE_CLIENT",
            "client",
            Some(client.id),
            None,
        )
        .await;

    Ok(Json(client))
}

pub async fn delete_client(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> Result<StatusCode, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::FORBIDDEN)?;
    let repo = ClientRepository::new(pool.clone());
    let audit_repo = AuditRepository::new(pool);

    let rows_affected = repo
        .soft_delete(id, tenant_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if rows_affected > 0 {
        // Log audit
        let _ = audit_repo
            .log(
                Some(tenant_id),
                Some(claims.sub),
                "DELETE_CLIENT",
                "client",
                Some(id),
                None,
            )
            .await;

        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
