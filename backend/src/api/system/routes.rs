use crate::api::system::controllers::email_check;
use crate::api::system::settings::{get_system_settings, update_system_settings};
use crate::core::rbac::middleware::require_super_admin;
use crate::core::tenant::middleware::tenant_middleware;
use axum::{
    middleware,
    routing::{get, post, put},
    Router,
};
use sqlx::PgPool;
use std::sync::Arc;

pub fn router(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/email-check", post(email_check))
        .route(
            "/settings",
            get(get_system_settings).put(update_system_settings),
        )
        .route_layer(middleware::from_fn(require_super_admin))
        .route_layer(middleware::from_fn_with_state(
            pool.clone(),
            tenant_middleware,
        ))
        .with_state(pool)
}
