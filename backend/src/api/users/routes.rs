use axum::{
    routing::{get, post, put, delete},
    Router,
    middleware,
};
use sqlx::PgPool;
use std::sync::Arc;
use super::controllers::{list_users, get_user, create_user, update_user, delete_user};
use crate::core::tenant::middleware::tenant_middleware;
use crate::core::rbac::middleware::require_tenant_admin;

pub fn router(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/", get(list_users).post(create_user))
        .route("/:id", get(get_user).put(update_user).delete(delete_user))
        .route_layer(middleware::from_fn(require_tenant_admin))
        .route_layer(middleware::from_fn(tenant_middleware))
        .with_state(pool)
}
