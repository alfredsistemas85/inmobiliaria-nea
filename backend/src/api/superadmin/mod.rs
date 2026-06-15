pub mod monitoring;
pub mod support;

use axum::Router;
use sqlx::PgPool;
use std::sync::Arc;

pub fn router(pool: Arc<PgPool>) -> Router {
    Router::new()
        .nest("/monitoring", monitoring::router(pool.clone()))
        .nest("/support", support::router(pool.clone()))
}
