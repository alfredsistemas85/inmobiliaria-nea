use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Appointment {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub client_id: Uuid,
    pub property_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub scheduled_at: DateTime<Utc>,
    pub status: Option<String>,
    pub notes: Option<String>,
    pub confirmed_at: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub confirmation_sent_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct AppointmentNotification {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub appointment_id: Uuid,
    pub notification_type: String,
    pub sent_at: Option<DateTime<Utc>>,
    pub status: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct AppointmentAuditLog {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub appointment_id: Uuid,
    pub action: String,
    pub old_status: Option<String>,
    pub new_status: Option<String>,
    pub performed_by: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}
