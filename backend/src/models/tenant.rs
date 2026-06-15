use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema, sqlx::FromRow)]
pub struct Tenant {
    pub id: Uuid,
    pub cuit: String,
    pub dni_responsable: String,
    pub first_name: String,
    pub last_name: String,
    pub business_name: String,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub is_active: Option<bool>,
    pub slug: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
