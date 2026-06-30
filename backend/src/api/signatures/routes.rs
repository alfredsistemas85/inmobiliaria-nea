use super::controllers;
use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use std::sync::Arc;
// use crate::core::rbac::middleware::require_tenant_admin; // To be added when RBAC is fully wired

pub fn router(pool: Arc<PgPool>) -> Router {
    let public_routes = Router::new()
        .route("/s/:token", get(controllers::get_public_info).post(controllers::submit_signature))
        .route("/s/verify/:code", get(controllers::verify_signature))
        .with_state(pool.clone());

    let admin_routes = Router::new()
        .route("/contracts/:id/signatures/request", post(controllers::request_signature))
        .route_layer(axum::middleware::from_fn_with_state(pool.clone(), crate::core::tenant::middleware::tenant_middleware))
        .with_state(pool.clone());

    Router::new().merge(public_routes).merge(admin_routes)
}
