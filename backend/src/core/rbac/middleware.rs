use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use crate::core::security::jwt::verify_jwt;

pub async fn require_role(role_required: &str, mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "));

    if let Some(token) = auth_header {
        match verify_jwt(token) {
            Ok(claims) => {
                if claims.role == "super_admin" || claims.role == role_required {
                    req.extensions_mut().insert(claims);
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
