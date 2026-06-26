pub mod controllers;

use crate::core::tenant::middleware::tenant_middleware;
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use std::sync::Arc;

pub fn router(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route(
            "/invoices",
            get(controllers::list_invoices).post(controllers::create_invoice),
        )
        .route(
            "/invoices/:id/pay_manual",
            post(controllers::mark_invoice_paid),
        )
        .route("/liquidations", get(controllers::generate_liquidations))
        .route_layer(middleware::from_fn_with_state(
            pool.clone(),
            tenant_middleware,
        ))
        .with_state(pool)
}
