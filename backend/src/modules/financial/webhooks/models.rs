use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct WebhookEvent {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub provider: String,
    pub event: String,
    pub payload: Value,
    pub processed: bool,
    pub processed_at: Option<DateTime<Utc>>,
    pub retries: i32,
    pub created_at: DateTime<Utc>,
}
