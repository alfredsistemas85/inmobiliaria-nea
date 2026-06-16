use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;
use std::sync::Arc;

use crate::api::appointments::controllers::{
    create_appointment, delete_appointment, get_appointment, list_appointments, update_appointment,
};
use crate::core::rbac::middleware::require_tenant_admin;
use crate::core::tenant::middleware::tenant_middleware;

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
        .route_layer(middleware::from_fn_with_state(pool.clone(), tenant_middleware))
        .with_state(pool)
}
