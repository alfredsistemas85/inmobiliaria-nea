use axum::{routing::get, Router};
use sqlx::PgPool;
use std::sync::Arc;

pub fn router(_pool: Arc<PgPool>) -> Router {
    Router::new()
        // Stub routes
        .route("/", get(|| async { "Clients endpoint" }))
}
