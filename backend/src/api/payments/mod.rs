pub mod controllers;

use axum::{
    routing::{post, put},
    Router,
};
use sqlx::PgPool;
use std::sync::Arc;

pub fn router(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route(
            "/checkout/subscription",
            post(controllers::create_subscription_preference),
        )
        .route(
            "/checkout/rent/:invoice_id",
            post(controllers::create_rent_preference),
        )
        .route("/config", put(controllers::update_payment_config))
        .route("/webhook", post(controllers::mp_webhook))
        .with_state(pool)
}
