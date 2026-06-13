use redis::AsyncCommands;
use std::sync::Arc;
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};

#[derive(Clone)]
pub struct RateLimitState {
    pub redis_client: redis::Client,
}

pub async fn rate_limit_middleware(
    State(state): State<Arc<RateLimitState>>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let ip = req
        .headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown_ip");

    match state.redis_client.get_multiplexed_async_connection().await {
        Ok(mut con) => {
            let key = format!("rate_limit:{}", ip);
            let incr_result: Result<i32, _> = con.incr(&key, 1).await;
            match incr_result {
                Ok(count) => {
                    if count == 1 {
                        let _: Result<(), _> = con.expire(&key, 60).await;
                    }
                    if count > 100 {
                        return Err(StatusCode::TOO_MANY_REQUESTS);
                    }
                }
                Err(e) => {
                    tracing::warn!("Redis incr failed, bypassing rate limit: {}", e);
                }
            }
        }
        Err(e) => {
            tracing::warn!("Redis connection failed, bypassing rate limit: {}", e);
        }
    }

    Ok(next.run(req).await)
}
