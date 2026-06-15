use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateAppointmentDto {
    pub client_id: Uuid,
    pub property_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub scheduled_at: DateTime<Utc>,
    pub status: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAppointmentDto {
    pub client_id: Option<Uuid>,
    pub property_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub status: Option<String>,
    pub notes: Option<String>,
}
