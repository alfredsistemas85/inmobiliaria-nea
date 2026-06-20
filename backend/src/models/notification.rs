use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Notification {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub user_id: Option<Uuid>,
    pub r#type: String, // 'NEW_LEAD', 'NEW_MESSAGE', 'ASSIGNED', 'APPOINTMENT_CREATED'
    pub title: String,
    pub content: String,
    pub read_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct NotificationListResponse {
    pub unread_count: i64,
    pub notifications: Vec<Notification>,
}
