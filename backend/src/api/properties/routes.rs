use axum::{
    routing::{get, post, delete},
    Router,
    middleware,
};
use sqlx::PgPool;
use std::sync::Arc;
use super::controllers::{list_properties, get_property, create_property, delete_property};
use crate::core::tenant::middleware::tenant_middleware;

pub fn router(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/", get(list_properties).post(create_property))
        .route("/:id", get(get_property).delete(delete_property))
        .route_layer(middleware::from_fn(tenant_middleware))
        .with_state(pool)
}
