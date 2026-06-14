use axum::{
    routing::get,
    Router,
    middleware,
};
use sqlx::PgPool;
use std::sync::Arc;

use crate::api::reports::controllers::{generate_leads_report, generate_appointments_report, generate_whatsapp_report};
use crate::core::tenant::middleware::tenant_middleware;

pub fn router(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/leads", get(generate_leads_report))
        .route("/appointments", get(generate_appointments_report))
        .route("/whatsapp", get(generate_whatsapp_report))
        .route_layer(middleware::from_fn(tenant_middleware))
        .with_state(pool)
}
