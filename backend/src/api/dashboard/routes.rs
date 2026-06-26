use axum::{middleware, routing::get, Router};
use sqlx::PgPool;
use std::sync::Arc;

use crate::api::dashboard::controllers::{get_activity, get_stats};
use crate::core::tenant::middleware::tenant_middleware;

pub fn router(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/stats", get(get_stats))
        .route("/activity", get(get_activity))
        .route_layer(middleware::from_fn_with_state(
            pool.clone(),
            tenant_middleware,
        ))
        .with_state(pool)
}
