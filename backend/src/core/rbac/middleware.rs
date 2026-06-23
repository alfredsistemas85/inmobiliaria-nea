use crate::core::security::jwt::Claims;
use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};

fn check_role(req: &Request, allowed_roles: &[&str]) -> Result<(), StatusCode> {
    if let Some(claims) = req.extensions().get::<Claims>() {
        if claims.role == "super_admin" || allowed_roles.contains(&claims.role.as_str()) {
            Ok(())
        } else {
            Err(StatusCode::FORBIDDEN)
        }
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

pub async fn require_super_admin(req: Request, next: Next) -> Result<Response, StatusCode> {
    check_role(&req, &[])?;
    Ok(next.run(req).await)
}

pub async fn require_tenant_admin(req: Request, next: Next) -> Result<Response, StatusCode> {
    check_role(&req, &["tenant_admin"])?;
    Ok(next.run(req).await)
}

pub async fn require_tenant_manager(req: Request, next: Next) -> Result<Response, StatusCode> {
    check_role(&req, &["tenant_admin", "tenant_manager"])?;
    Ok(next.run(req).await)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use uuid::Uuid;

    fn create_req(role: &str, tenant_id: Option<Uuid>) -> Request {
        let mut req = Request::new(Body::empty());
        req.extensions_mut().insert(Claims {
            sub: Uuid::new_v4(),
            tenant_id,
            role: role.to_string(),
            email_verified: true,
            exp: 9999999999,
            token_type: "access".to_string(),
        });
        req
    }

    #[test]
    fn test_tenant_agent_access_denied() {
        let req = create_req("tenant_agent", Some(Uuid::new_v4()));
        assert_eq!(
            check_role(&req, &["tenant_admin"]),
            Err(StatusCode::FORBIDDEN)
        );
        assert_eq!(check_role(&req, &[]), Err(StatusCode::FORBIDDEN));
    }

    #[test]
    fn test_super_admin_access_allowed() {
        let req = create_req("super_admin", None);
        assert_eq!(check_role(&req, &["tenant_admin"]), Ok(()));
        assert_eq!(check_role(&req, &[]), Ok(()));
    }
}
