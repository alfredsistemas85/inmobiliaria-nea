use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use crate::core::security::jwt::verify_jwt;
use super::extractor::TenantId;

pub async fn tenant_middleware(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "));

    if let Some(token) = auth_header {
        match verify_jwt(token) {
            Ok(claims) => {
                req.extensions_mut().insert(claims.clone());
                if let Some(tenant_id) = claims.tenant_id {
                    req.extensions_mut().insert(TenantId(tenant_id));
                    Ok(next.run(req).await)
                } else if claims.role == "super_admin" {
                    // super_admin doesn't have a specific tenant, but can access global routes
                    // If a route explicitly requires TenantId, it will fail at the extractor level, which is desired for tenant-specific routes.
                    Ok(next.run(req).await)
                } else {
                    Err(StatusCode::FORBIDDEN)
                }
            }
            Err(_) => Err(StatusCode::UNAUTHORIZED),
        }
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
