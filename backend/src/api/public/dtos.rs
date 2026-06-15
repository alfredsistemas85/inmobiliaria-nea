use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct BootstrapResponse {
    pub tenant: TenantInfo,
    pub portal: PortalConfig,
}

#[derive(Serialize)]
pub struct TenantInfo {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub logo_url: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub whatsapp: Option<String>,
    pub address: Option<String>,
}

#[derive(Serialize)]
pub struct PortalConfig {
    pub allow_contact_form: bool,
    pub allow_whatsapp: bool,
}

#[derive(Deserialize)]
pub struct PublicPropertyFilter {
    pub tenant_id: Uuid,
    pub operation_type: Option<String>,
    pub property_type: Option<String>,
    pub city: Option<String>,
    pub price_min: Option<rust_decimal::Decimal>,
    pub price_max: Option<rust_decimal::Decimal>,
    pub bedrooms: Option<i32>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Serialize)]
pub struct PublicPropertyResponse {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub property_type: String,
    pub operation_type: String,
    pub price: rust_decimal::Decimal,
    pub currency: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub square_meters: Option<rust_decimal::Decimal>,
    pub bedrooms: Option<i32>,
    pub bathrooms: Option<i32>,
    pub features: Option<sqlx::types::Json<serde_json::Value>>,
    pub main_image_url: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize)]
pub struct PublicPropertyDetailResponse {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub property_type: String,
    pub operation_type: String,
    pub price: rust_decimal::Decimal,
    pub currency: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub square_meters: Option<rust_decimal::Decimal>,
    pub bedrooms: Option<i32>,
    pub bathrooms: Option<i32>,
    pub features: Option<sqlx::types::Json<serde_json::Value>>,
    pub images: Vec<crate::models::property::PropertyImage>,
    pub documents: Vec<crate::models::property::PropertyDocument>,
    pub agent: Option<PublicAgentInfo>,
}

#[derive(Serialize)]
pub struct PublicAgentInfo {
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
    pub email: String,
}

#[derive(Deserialize)]
pub struct CreatePublicLeadDto {
    pub tenant_id: Uuid,
    pub property_id: Option<Uuid>,
    pub name: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub message: Option<String>,
    // Honeypot field
    pub website: Option<String>,
}
