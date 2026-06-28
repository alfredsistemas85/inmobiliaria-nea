use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TreasuryAccount {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    #[sqlx(rename = "type")]
    pub account_type: String, // e.g. BANK, CASH
    pub currency: String,
    pub current_balance: Decimal,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TreasuryMovement {
    pub id: Uuid,
    pub account_id: Uuid,
    pub movement_type: String, // IN, OUT
    pub amount: Decimal,
    pub reference: Option<String>,
    pub description: String,
    pub created_at: DateTime<Utc>,
}
