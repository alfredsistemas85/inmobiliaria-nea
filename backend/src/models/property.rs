use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use sqlx::types::Json;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, ToSchema, sqlx::FromRow)]
pub struct Property {
    pub id: Uuid,
    pub tenant_id: Uuid,
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
    pub features: Option<Json<Value>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, sqlx::FromRow)]
pub struct PropertyImage {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub property_id: Uuid,
    pub url: String,
    pub is_main: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, sqlx::FromRow)]
pub struct PropertyDocument {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub property_id: Uuid,
    pub title: Option<String>,
    pub url: String,
    pub created_at: Option<DateTime<Utc>>,
}
