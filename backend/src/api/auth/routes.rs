use axum::{
    routing::{get, post},
    Router,
    middleware,
};
use sqlx::PgPool;
use std::sync::Arc;
use super::controllers::{login, refresh, logout, change_password, me};
use crate::core::tenant::middleware::tenant_middleware;

pub fn router(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/refresh", post(refresh))
        .route("/logout", post(logout))
        .route("/me", get(me).route_layer(middleware::from_fn(tenant_middleware)))
        .route("/change-password", post(change_password).route_layer(middleware::from_fn(tenant_middleware)))
        .with_state(pool)
}
