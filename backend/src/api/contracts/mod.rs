pub mod controllers;

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
        .with_state(pool)
}
