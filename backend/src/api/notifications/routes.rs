use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use std::sync::Arc;

use crate::api::notifications::controllers::{list_notifications, mark_as_read};
use crate::core::tenant::middleware::tenant_middleware;

pub fn router(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/", get(list_notifications))
        .route("/:id/read", post(mark_as_read))
        .route_layer(middleware::from_fn_with_state(
            pool.clone(),
            tenant_middleware,
        ))
        .with_state(pool)
}
