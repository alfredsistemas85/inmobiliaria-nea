use super::controllers::list_roles;
use crate::core::rbac::middleware::require_super_admin;
use crate::core::tenant::middleware::tenant_middleware;
use axum::{middleware, routing::get, Router};
use sqlx::PgPool;
use std::sync::Arc;

pub fn router(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/", get(list_roles))
        .route_layer(middleware::from_fn(require_super_admin))
        .route_layer(middleware::from_fn_with_state(pool.clone(), tenant_middleware))
        .with_state(pool)
}
