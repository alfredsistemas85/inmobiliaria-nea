pub mod controllers;
pub mod models;
pub mod dto;
pub mod adjustments_controllers;

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
        .route("/", get(controllers::list_contracts).post(controllers::create_contract))
        .route("/v2", post(controllers::create_contract_v2))
        .route("/:id/pdf", get(controllers::generate_contract_pdf))
        .route("/adjustments/pending", get(adjustments_controllers::list_pending_adjustments))
        .route("/:id/adjustments", get(adjustments_controllers::list_adjustments))
        .route("/:id/installments", get(adjustments_controllers::list_installments))
        .route("/:id/adjustments/propose", post(adjustments_controllers::propose_adjustment))
        .route("/adjustments/:adj_id/approve", post(adjustments_controllers::approve_adjustment))
        .route("/adjustments/:adj_id/reject", post(adjustments_controllers::reject_adjustment))
        .route_layer(middleware::from_fn_with_state(pool.clone(), tenant_middleware))
        .with_state(pool)
}
