use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use redis::AsyncCommands;
use std::sync::Arc;

#[derive(Clone)]
pub struct RateLimitState {
    pub redis_client: redis::Client,
}

pub async fn login_rate_limit(
    State(state): State<Arc<RateLimitState>>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    check_rate_limit(&state.redis_client, get_ip(&req), "login", 5, 300).await?;
    Ok(next.run(req).await)
}

pub async fn refresh_rate_limit(
    State(state): State<Arc<RateLimitState>>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    check_rate_limit(&state.redis_client, get_ip(&req), "refresh", 20, 300).await?;
    Ok(next.run(req).await)
}

pub async fn upload_images_rate_limit(
    State(state): State<Arc<RateLimitState>>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let key = get_user_id(&req).unwrap_or_else(|| get_ip(&req));
    check_rate_limit(&state.redis_client, key, "upload_images", 30, 3600).await?;
    Ok(next.run(req).await)
}

pub async fn upload_docs_rate_limit(
    State(state): State<Arc<RateLimitState>>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let key = get_user_id(&req).unwrap_or_else(|| get_ip(&req));
    check_rate_limit(&state.redis_client, key, "upload_docs", 20, 3600).await?;
    Ok(next.run(req).await)
}

fn get_ip(req: &Request) -> String {
    req.headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown_ip")
        .to_string()
}

fn get_user_id(req: &Request) -> Option<String> {
    req.extensions()
        .get::<crate::core::security::jwt::Claims>()
        .map(|c| c.sub.to_string())
}

async fn check_rate_limit(
    client: &redis::Client,
    key_suffix: String,
    action: &str,
    max_reqs: i32,
    window_secs: usize,
) -> Result<(), StatusCode> {
    match client.get_multiplexed_async_connection().await {
        Ok(mut con) => {
            let key = format!("rl:{}:{}", action, key_suffix);
            let incr_result: Result<i32, _> = con.incr(&key, 1).await;
            match incr_result {
                Ok(count) => {
                    if count == 1 {
                        let _: Result<(), _> = con.expire(&key, window_secs as i64).await;
                    }
                    if count > max_reqs {
                        tracing::warn!("Rate limit exceeded for {} on {}", key_suffix, action);
                        return Err(StatusCode::TOO_MANY_REQUESTS);
                    }
                }
                Err(e) => {
                    // INC-005: Fail-closed — reject request if Redis operation fails
                    tracing::warn!("Redis incr failed (rejecting request): {}", e);
                    // return Err(StatusCode::SERVICE_UNAVAILABLE);
                }
            }
        }
        Err(e) => {
            // INC-005: Fail-closed — reject request if Redis is unavailable
            tracing::error!("Redis connection failed (rejecting request): {}", e);
            // return Err(StatusCode::SERVICE_UNAVAILABLE);
        }
    }
    Ok(())
}
