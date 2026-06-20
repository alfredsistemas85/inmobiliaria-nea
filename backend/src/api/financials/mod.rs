pub mod controllers;

use axum::{
    routing::{get, post},
    Router,
    middleware,
};
use std::sync::Arc;
use sqlx::PgPool;
use crate::core::tenant::middleware::tenant_middleware;

pub fn router(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/invoices", get(controllers::list_invoices).post(controllers::create_invoice))
        .route("/invoices/:id/pay_manual", post(controllers::mark_invoice_paid))
        .route("/liquidations", get(controllers::generate_liquidations))
        .route_layer(middleware::from_fn_with_state(pool.clone(), tenant_middleware))
        .with_state(pool)
}
