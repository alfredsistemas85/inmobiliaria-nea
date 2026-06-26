use super::controllers;
use crate::core::tenant::middleware::tenant_middleware;
use axum::{
    middleware,
    routing::{delete, get, post},
    Router,
};
use sqlx::PgPool;
use std::sync::Arc;

pub fn routes(pool: Arc<PgPool>) -> Router {
    let protected_routes = Router::new()
        .route("/upload-url", post(controllers::generate_upload_url))
        .route(
            "/entity/:entity_type/:entity_id",
            get(controllers::list_entity_documents),
        )
        .route("/:id", get(controllers::get_document))
        .route("/:id", delete(controllers::delete_document))
        .route_layer(middleware::from_fn_with_state(
            pool.clone(),
            tenant_middleware,
        ));

    Router::new()
        .route("/:id/view", get(controllers::view_document))
        .merge(protected_routes)
        .with_state(pool)
}
