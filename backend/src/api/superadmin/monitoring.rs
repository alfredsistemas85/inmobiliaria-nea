use crate::core::rbac::middleware::require_super_admin;
use axum::{
    extract::State, http::StatusCode, middleware, response::IntoResponse, routing::get, Json,
    Router,
};
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Serialize, sqlx::FromRow)]
pub struct SystemError {
    pub id: Uuid,
    pub tenant_id: Option<Uuid>,
    pub error_type: String,
    pub endpoint: Option<String>,
    pub method: Option<String>,
    pub error_message: String,
    pub stack_trace: Option<String>,
    pub payload: Option<serde_json::Value>,
    pub resolved: Option<bool>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolved_by: Option<Uuid>,
    pub created_at: Option<DateTime<Utc>>,
}

async fn get_system_errors(
    State(pool): State<Arc<PgPool>>,
) -> Result<Json<Vec<SystemError>>, StatusCode> {
    let errors = sqlx::query_as::<_, SystemError>(
        "SELECT * FROM system_errors ORDER BY created_at DESC LIMIT 100",
    )
    .fetch_all(&*pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(errors))
}

pub fn router(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/errors", get(get_system_errors))
        .route_layer(middleware::from_fn(require_super_admin))
        .route_layer(middleware::from_fn_with_state(
            pool.clone(),
            crate::core::tenant::middleware::tenant_middleware,
        ))
        .with_state(pool)
}
