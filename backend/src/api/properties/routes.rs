use axum::{
    routing::{get, post, delete},
    Router,
    middleware,
};
use sqlx::PgPool;
use std::sync::Arc;
use super::controllers::{list_properties, get_property, create_property, delete_property};
use crate::core::tenant::middleware::tenant_middleware;
use crate::core::rbac::middleware::require_tenant_admin;

pub fn router(pool: Arc<PgPool>) -> Router {
    let agent_routes = Router::new()
        .route("/", get(list_properties))
        .route("/:id", get(get_property));

    let admin_routes = Router::new()
        .route("/", post(create_property))
        .route("/:id", delete(delete_property))
        .route_layer(middleware::from_fn(require_tenant_admin));

    Router::new()
        .merge(agent_routes)
        .merge(admin_routes)
        .route_layer(middleware::from_fn(tenant_middleware))
        .with_state(pool)
}
