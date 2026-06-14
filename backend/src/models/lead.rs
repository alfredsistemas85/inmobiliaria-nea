use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Lead {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub client_id: Uuid,
    pub property_id: Option<Uuid>,
    pub status: Option<String>,
    pub source: Option<String>,
    pub assigned_to: Option<Uuid>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct LeadActivity {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub lead_id: Uuid,
    pub user_id: Option<Uuid>,
    pub activity_type: String,
    pub description: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}
