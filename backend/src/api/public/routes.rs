use crate::api::public::controllers::{
    bootstrap, create_public_lead, get_featured, get_properties, get_property,
};
use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct PublicState {
    pub pool: Arc<PgPool>,
    pub redis: Arc<redis::Client>,
}

pub fn router(pool: Arc<PgPool>, redis: Arc<redis::Client>) -> Router {
    let state = PublicState { pool, redis };
    Router::new()
        .route("/bootstrap", get(bootstrap))
        .route("/properties", get(get_properties))
        .route("/properties/:id", get(get_property))
        .route("/featured", get(get_featured))
        .route("/leads", post(create_public_lead))
        .with_state(state)
}
