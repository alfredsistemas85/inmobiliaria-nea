use crate::core::rbac::middleware::require_super_admin;
use axum::{
    extract::State,
    http::StatusCode,
    middleware,
    routing::get,
    Json, Router,
};
use serde::Serialize;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Serialize, sqlx::FromRow)]
pub struct SupportTicket {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub user_id: Option<Uuid>,
    pub subject: String,
    pub status: Option<String>,
    pub priority: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

async fn get_all_tickets(
    State(pool): State<Arc<PgPool>>,
) -> Result<Json<Vec<SupportTicket>>, StatusCode> {
    let tickets = sqlx::query_as::<_, SupportTicket>(
        "SELECT * FROM support_tickets ORDER BY created_at DESC",
    )
    .fetch_all(&*pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(tickets))
}

pub fn router(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/", get(get_all_tickets))
        .route_layer(middleware::from_fn(require_super_admin))
        .route_layer(middleware::from_fn_with_state(pool.clone(), crate::core::tenant::middleware::tenant_middleware))
        .with_state(pool)
}
