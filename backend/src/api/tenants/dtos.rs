use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateTenantDto {
    pub business_name: String,
    pub cuit: String,
    pub admin_email: String,
    pub admin_first_name: String,
    pub admin_last_name: String,
    pub phone: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TenantResponseDto {
    pub id: Uuid,
    pub cuit: String,
    pub dni_responsable: String,
    pub first_name: String,
    pub last_name: String,
    pub business_name: String,
    pub is_active: Option<bool>,
}

impl From<crate::models::tenant::Tenant> for TenantResponseDto {
    fn from(tenant: crate::models::tenant::Tenant) -> Self {
        Self {
            id: tenant.id,
            cuit: tenant.cuit,
            dni_responsable: tenant.dni_responsable,
            first_name: tenant.first_name,
            last_name: tenant.last_name,
            business_name: tenant.business_name,
            is_active: tenant.is_active,
        }
    }
}
