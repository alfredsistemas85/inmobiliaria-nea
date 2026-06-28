use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct NotificationQueue {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub installment_id: Option<Uuid>,
    pub channel: String, // e.g. WHATSAPP, EMAIL
    pub scheduled_at: DateTime<Utc>,
    pub sent_at: Option<DateTime<Utc>>,
    pub status: String, // PENDING, SENT, FAILED
    pub created_at: DateTime<Utc>,
}
