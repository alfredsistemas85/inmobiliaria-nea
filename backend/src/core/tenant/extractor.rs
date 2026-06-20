use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct TenantId(pub Uuid);

#[async_trait]
impl<S> FromRequestParts<S> for TenantId
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if let Some(tenant_id) = parts.extensions.get::<TenantId>() {
            Ok(tenant_id.clone())
        } else {
            Err((StatusCode::UNAUTHORIZED, "Tenant context missing"))
        }
    }
}
