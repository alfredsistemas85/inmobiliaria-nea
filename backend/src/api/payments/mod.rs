pub mod controllers;

use axum::{
    routing::post,
    Router,
};
use std::sync::Arc;
use sqlx::PgPool;

pub fn router(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/checkout/subscription", post(controllers::create_subscription_preference))
        .route("/checkout/rent/:invoice_id", post(controllers::create_rent_preference))
        .route("/webhook", post(controllers::mp_webhook))
        .with_state(pool)
}
