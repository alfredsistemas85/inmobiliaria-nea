use super::controllers::{change_password, login, logout, me, refresh};
use crate::core::tenant::middleware::tenant_middleware;
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use std::sync::Arc;

pub fn router(
    pool: Arc<PgPool>,
    rl_state: Arc<crate::core::security::rate_limit::RateLimitState>,
) -> Router {
    Router::new()
        .route(
            "/login",
            post(login).route_layer(middleware::from_fn_with_state(
                rl_state.clone(),
                crate::core::security::rate_limit::login_rate_limit,
            )),
        )
        .route(
            "/refresh",
            post(refresh).route_layer(middleware::from_fn_with_state(
                rl_state,
                crate::core::security::rate_limit::refresh_rate_limit,
            )),
        )
        .route("/logout", post(logout))
        .route(
            "/me",
            get(me).route_layer(middleware::from_fn(tenant_middleware)),
        )
        .route(
            "/change-password",
            post(change_password).route_layer(middleware::from_fn(tenant_middleware)),
        )
        .with_state(pool)
}
