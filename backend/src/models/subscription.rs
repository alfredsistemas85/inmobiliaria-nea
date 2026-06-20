use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema, sqlx::Type, Clone, PartialEq)]
#[sqlx(type_name = "plan_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PlanType {
    Basic,
    Pro,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, sqlx::Type, Clone, PartialEq)]
#[sqlx(type_name = "subscription_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SubscriptionStatus {
    Trial,
    Active,
    Suspended,
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, sqlx::FromRow)]
pub struct Subscription {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub plan_type: PlanType,
    pub status: SubscriptionStatus,
    pub starts_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub trial_ends_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
