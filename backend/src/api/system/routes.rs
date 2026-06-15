use crate::api::system::controllers::email_check;
use crate::core::rbac::middleware::require_super_admin;
use crate::core::tenant::middleware::tenant_middleware;
use axum::{middleware, routing::post, Router};
use sqlx::PgPool;
use std::sync::Arc;

pub fn router(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/email-check", post(email_check))
        .route_layer(middleware::from_fn(require_super_admin))
        .route_layer(middleware::from_fn(tenant_middleware))
        .with_state(pool)
}
