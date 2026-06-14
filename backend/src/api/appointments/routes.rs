use axum::{
    routing::{get, post, put, delete},
    Router,
    middleware,
};
use sqlx::PgPool;
use std::sync::Arc;

use crate::api::appointments::controllers::{
    list_appointments, get_appointment, create_appointment, update_appointment, delete_appointment,
};
use crate::core::tenant::middleware::tenant_middleware;
use crate::core::rbac::middleware::require_tenant_admin;

pub fn router(pool: Arc<PgPool>) -> Router {
    let agent_routes = Router::new()
        .route("/", get(list_appointments).post(create_appointment))
        .route("/:id", get(get_appointment).put(update_appointment));

    let admin_routes = Router::new()
        .route("/:id", delete(delete_appointment))
        .route_layer(middleware::from_fn(require_tenant_admin));

    Router::new()
        .merge(agent_routes)
        .merge(admin_routes)
        .route_layer(middleware::from_fn(tenant_middleware))
        .with_state(pool)
}
