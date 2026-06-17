pub mod controllers;
pub mod models;
pub mod dto;
pub mod adjustments_controllers;

use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use sqlx::PgPool;

pub fn router(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/", get(controllers::list_contracts).post(controllers::create_contract))
        .route("/:id/pdf", get(controllers::generate_contract_pdf))
        .route("/:id/adjustments", get(adjustments_controllers::list_adjustments))
        .route("/:id/installments", get(adjustments_controllers::list_installments))
        .route("/:id/adjustments/propose", post(adjustments_controllers::propose_adjustment))
        .route("/adjustments/:adj_id/approve", post(adjustments_controllers::approve_adjustment))
        .route("/adjustments/:adj_id/rollback", post(adjustments_controllers::rollback_adjustment))
        .with_state(pool)
}
