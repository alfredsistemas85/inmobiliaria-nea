use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;
use std::sync::Arc;

use crate::api::leads::controllers::{
    convert_lead, create_lead, delete_lead, get_lead, list_leads, update_lead,
};
use crate::core::rbac::middleware::require_tenant_admin;
use crate::core::tenant::middleware::tenant_middleware;

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
        .route_layer(middleware::from_fn_with_state(
            pool.clone(),
            tenant_middleware,
        ))
        .with_state(pool)
}
