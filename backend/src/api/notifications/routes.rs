use axum::{
    routing::{get, post},
    Router,
    middleware,
};
use sqlx::PgPool;
use std::sync::Arc;

use crate::api::notifications::controllers::{list_notifications, mark_as_read};
use crate::core::tenant::middleware::tenant_middleware;

pub fn router(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/", get(list_notifications))
        .route("/:id/read", post(mark_as_read))
        .route_layer(middleware::from_fn(tenant_middleware))
        .with_state(pool)
}
