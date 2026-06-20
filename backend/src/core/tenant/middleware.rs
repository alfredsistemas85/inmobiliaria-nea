use super::extractor::TenantId;
use crate::core::security::jwt::verify_jwt;
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json, Extension,
};
use std::sync::Arc;
use sqlx::PgPool;
use serde_json::json;

pub async fn tenant_middleware(
    State(pool): State<Arc<PgPool>>,
    Extension(redis_client): Extension<Arc<redis::Client>>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "));

    if let Some(token) = auth_header {
        match verify_jwt(token) {
            Ok(claims) => {
                req.extensions_mut().insert(claims.clone());
                
                if claims.role == "super_admin" || claims.role == "SUPERADMIN" {
                    return Ok(next.run(req).await);
                }

                if let Some(tenant_id) = claims.tenant_id {
                    let redis_key = format!("subscription:{}", tenant_id);
                    let mut cached_status = None;

                    // 1. Try Redis
                    if let Ok(mut redis_conn) = redis_client.get_multiplexed_async_connection().await {
                        if let Ok(st) = redis::cmd("GET").arg(&redis_key).query_async::<_, Option<String>>(&mut redis_conn).await {
                            cached_status = st;
                        }
                    }

                    let status = match cached_status {
                        Some(st) => st,
                        None => {
                            // 2. Fallback to PostgreSQL
                            let sub = sqlx::query!("SELECT status::text FROM subscriptions WHERE tenant_id = $1", tenant_id)
                                .fetch_optional(&*pool).await.map_err(|_: sqlx::Error| StatusCode::INTERNAL_SERVER_ERROR)?;
                            
                            let st = sub.and_then(|s| s.status).unwrap_or_default();
                            
                            // Try to save to cache without blocking
                            if let Ok(mut redis_conn) = redis_client.get_multiplexed_async_connection().await {
                                let _: Result<(), _> = redis::cmd("SETEX").arg(&redis_key).arg(300).arg(&st).query_async(&mut redis_conn).await;
                            }
                            st
                        }
                    };

                    if status == "SUSPENDED" || status == "CANCELLED" || status.is_empty() {
                        let error_response = (
                            StatusCode::FORBIDDEN,
                            Json(json!({
                                "error": "subscription_blocked",
                                "message": "La suscripción de la inmobiliaria se encuentra suspendida o cancelada."
                            }))
                        ).into_response();
                        return Ok(error_response);
                    }

                    req.extensions_mut().insert(TenantId(tenant_id));
                    return Ok(next.run(req).await);
                } else {
                    return Err(StatusCode::FORBIDDEN);
                }
            }
            Err(_) => return Err(StatusCode::UNAUTHORIZED),
        }
    }
    
    Err(StatusCode::UNAUTHORIZED)
}
