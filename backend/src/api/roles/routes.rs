use axum::{
    routing::get,
    Router,
    middleware,
};
use sqlx::PgPool;
use std::sync::Arc;
use super::controllers::list_roles;
use crate::core::tenant::middleware::tenant_middleware;

pub fn router(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/", get(list_roles))
        .route_layer(middleware::from_fn(tenant_middleware))
        .with_state(pool)
}
