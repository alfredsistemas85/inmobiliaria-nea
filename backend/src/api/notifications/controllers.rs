use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension,
    Json,
};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    core::security::jwt::Claims,
    infrastructure::database::notifications::NotificationRepository,
    models::notification::NotificationListResponse,
};

pub async fn list_notifications(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<NotificationListResponse>, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::FORBIDDEN)?;
    let user_id = claims.sub;

    let repo = NotificationRepository::new(pool);
    let notifications = repo.list(tenant_id, user_id, 20).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let unread_count = repo.count_unread(tenant_id, user_id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(NotificationListResponse {
        unread_count,
        notifications,
    }))
}

pub async fn mark_as_read(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::FORBIDDEN)?;
    let user_id = claims.sub;

    let repo = NotificationRepository::new(pool);
    repo.mark_as_read(tenant_id, id, user_id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}
