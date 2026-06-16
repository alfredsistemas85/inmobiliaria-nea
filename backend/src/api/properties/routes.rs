use super::controllers::{
    create_property, delete_property, get_property, list_properties, update_property,
    upload_document, upload_image,
};
use crate::core::rbac::middleware::require_tenant_admin;
use crate::core::tenant::middleware::tenant_middleware;
use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;
use std::sync::Arc;

pub fn router(
    pool: Arc<PgPool>,
    rl_state: Arc<crate::core::security::rate_limit::RateLimitState>,
) -> Router {
    let agent_routes = Router::new()
        .route("/", get(list_properties))
        .route("/:id", get(get_property));

    let admin_routes = Router::new()
        .route("/", post(create_property))
        .route("/:id", put(update_property).delete(delete_property))
        .route(
            "/:id/images",
            post(upload_image).route_layer(middleware::from_fn_with_state(
                rl_state.clone(),
                crate::core::security::rate_limit::upload_images_rate_limit,
            )),
        )
        .route(
            "/:id/documents",
            post(upload_document).route_layer(middleware::from_fn_with_state(
                rl_state,
                crate::core::security::rate_limit::upload_docs_rate_limit,
            )),
        )
        .route_layer(middleware::from_fn(require_tenant_admin));

    Router::new()
        .merge(agent_routes)
        .merge(admin_routes)
        .route_layer(middleware::from_fn_with_state(pool.clone(), tenant_middleware))
        .with_state(pool)
}
