pub mod controllers;

use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use sqlx::PgPool;

pub fn router(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/invoices", get(controllers::list_invoices).post(controllers::create_invoice))
        .route("/invoices/:id/pay_manual", post(controllers::mark_invoice_paid))
        .route("/liquidations", get(controllers::generate_liquidations))
        .with_state(pool)
}
