pub mod dashboard;
pub mod monitoring;
pub mod scheduler;
pub mod subscriptions;
pub mod support;
pub mod tenants;

use axum::Router;
use sqlx::PgPool;
use std::sync::Arc;

pub fn router(pool: Arc<PgPool>) -> Router {
    Router::new()
        .nest("/monitoring", monitoring::router(pool.clone()))
        .nest("/support", support::router(pool.clone()))
        .nest("/subscriptions", subscriptions::router(pool.clone()))
        .nest("/dashboard", dashboard::router(pool.clone()))
        .nest("/tenants", tenants::router(pool.clone()))
        .nest("/scheduler", scheduler::router(pool.clone()))
}
