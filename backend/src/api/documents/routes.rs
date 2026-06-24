use axum::{
    middleware,
    routing::{get, post, delete},
    Router,
};
use std::sync::Arc;
use sqlx::PgPool;
use super::controllers;
use crate::core::tenant::middleware::tenant_middleware;

pub fn routes(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/upload-url", post(controllers::generate_upload_url))
        .route("/entity/:entity_type/:entity_id", get(controllers::list_entity_documents))
        .route("/:id", get(controllers::get_document))
        .route("/:id", delete(controllers::delete_document))
        .route_layer(middleware::from_fn_with_state(pool.clone(), tenant_middleware))
        .with_state(pool)
}
