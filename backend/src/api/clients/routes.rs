use axum::{
    routing::{get, post, put, delete},
    Router,
    middleware,
};
use sqlx::PgPool;
use std::sync::Arc;

use crate::api::clients::controllers::{
    list_clients, get_client, create_client, update_client, delete_client,
};
use crate::core::tenant::middleware::tenant_middleware;
use crate::core::rbac::middleware::require_tenant_admin;

pub fn router(pool: Arc<PgPool>) -> Router {
    let agent_routes = Router::new()
        .route("/", get(list_clients).post(create_client))
        .route("/:id", get(get_client).put(update_client));

    let admin_routes = Router::new()
        .route("/:id", delete(delete_client))
        .route_layer(middleware::from_fn(require_tenant_admin));

    Router::new()
        .merge(agent_routes)
        .merge(admin_routes)
        .route_layer(middleware::from_fn(tenant_middleware))
        .with_state(pool)
}
