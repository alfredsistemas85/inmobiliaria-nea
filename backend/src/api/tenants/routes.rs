use axum::{
    routing::get,
    Router,
    middleware,
};
use sqlx::PgPool;
use std::sync::Arc;
use super::controllers::{list_tenants, get_tenant, create_tenant};
use crate::core::tenant::middleware::tenant_middleware;
use crate::core::rbac::middleware::require_super_admin;

pub fn router(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/", get(list_tenants).post(create_tenant))
        .route("/:id", get(get_tenant))
        .route_layer(middleware::from_fn(require_super_admin))
        .route_layer(middleware::from_fn(tenant_middleware))
        .with_state(pool)
}
