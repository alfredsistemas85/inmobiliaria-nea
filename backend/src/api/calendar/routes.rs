use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use sqlx::PgPool;
use super::controllers;

pub fn routes(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/google/connect", get(controllers::google_connect))
        .route("/google/callback", get(controllers::google_callback))
        .route("/status", get(controllers::get_status))
        .route("/disconnect", post(controllers::disconnect))
        .with_state(pool)
}
