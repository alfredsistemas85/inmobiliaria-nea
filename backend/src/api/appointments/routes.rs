use axum::{routing::{get, post}, Router};
use sqlx::PgPool;
use std::sync::Arc;

pub fn router(_pool: Arc<PgPool>) -> Router {
    Router::new()
        // Stub routes
        .route("/", get(|| async { "Appointments endpoint" }))
        // Webhook for Evolution API
        .route("/webhook/evolution", post(|| async { "Evolution Webhook" }))
}
