use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use sqlx::types::Json;
use serde_json::Value;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreatePropertyDto {
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
    pub status: Option<String>,
    pub features: Option<Value>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PropertyResponseDto {
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
    pub status: Option<String>,
    pub features: Option<Value>,
}

impl From<crate::models::property::Property> for PropertyResponseDto {
    fn from(prop: crate::models::property::Property) -> Self {
        Self {
            id: prop.id,
            title: prop.title,
            description: prop.description,
            property_type: prop.property_type,
            operation_type: prop.operation_type,
            price: prop.price,
            currency: prop.currency,
            address: prop.address,
            city: prop.city,
            province: prop.province,
            square_meters: prop.square_meters,
            bedrooms: prop.bedrooms,
            bathrooms: prop.bathrooms,
            status: prop.status,
            features: prop.features.map(|j| j.0),
        }
    }
}
