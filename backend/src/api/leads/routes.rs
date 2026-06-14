use axum::{
    routing::{get, post, put, delete},
    Router,
    middleware,
};
use sqlx::PgPool;
use std::sync::Arc;

use crate::api::leads::controllers::{
    list_leads, get_lead, create_lead, update_lead, convert_lead, delete_lead,
};
use crate::core::tenant::middleware::tenant_middleware;
use crate::core::rbac::middleware::require_tenant_admin;

pub fn router(pool: Arc<PgPool>) -> Router {
    let agent_routes = Router::new()
        .route("/", get(list_leads).post(create_lead))
        .route("/:id", get(get_lead).put(update_lead))
        .route("/:id/convert", post(convert_lead));

    let admin_routes = Router::new()
        .route("/:id", delete(delete_lead))
        .route_layer(middleware::from_fn(require_tenant_admin));

    Router::new()
        .merge(agent_routes)
        .merge(admin_routes)
        .route_layer(middleware::from_fn(tenant_middleware))
        .with_state(pool)
}
