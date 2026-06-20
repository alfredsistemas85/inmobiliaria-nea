pub mod controllers;
pub mod routes;

use std::sync::Arc;
use sqlx::PgPool;
use axum::Router;

pub fn router(pool: Arc<PgPool>) -> Router {
    routes::routes(pool)
}
