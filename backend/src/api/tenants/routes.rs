use super::controllers::{create_tenant, get_tenant, list_tenants};
use crate::core::rbac::middleware::require_super_admin;
use crate::core::tenant::middleware::tenant_middleware;
use axum::{middleware, routing::get, Router};
use sqlx::PgPool;
use std::sync::Arc;

pub fn router(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/", get(list_tenants).post(create_tenant))
        .route("/:id", get(get_tenant))
        .route_layer(middleware::from_fn(require_super_admin))
        .route_layer(middleware::from_fn(tenant_middleware))
        .with_state(pool)
}
