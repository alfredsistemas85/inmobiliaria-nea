pub mod controllers;
pub mod routes;
pub mod storage;

use axum::Router;
use sqlx::PgPool;
use std::sync::Arc;

pub fn router(pool: Arc<PgPool>) -> Router {
    routes::routes(pool)
}
