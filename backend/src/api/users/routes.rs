use super::controllers::{create_user, delete_user, get_user, list_users, update_user};
use crate::core::rbac::middleware::require_tenant_admin;
use crate::core::tenant::middleware::tenant_middleware;
use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;
use std::sync::Arc;

pub fn router(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/", get(list_users).post(create_user))
        .route("/:id", get(get_user).put(update_user).delete(delete_user))
        .route_layer(middleware::from_fn(require_tenant_admin))
        .route_layer(middleware::from_fn_with_state(pool.clone(), tenant_middleware))
        .with_state(pool)
}
