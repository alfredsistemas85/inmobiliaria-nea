use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;
use std::sync::Arc;

use crate::api::clients::controllers::{
    create_client, delete_client, get_client, list_clients, update_client,
};
use crate::core::rbac::middleware::require_tenant_admin;
use crate::core::tenant::middleware::tenant_middleware;

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
