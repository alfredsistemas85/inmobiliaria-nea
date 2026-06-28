use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashBox {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub type_: String, // "PettyCash", "BankAccount"
    pub currency: String,
    pub opening_balance: Decimal,
    pub current_balance: Decimal,
    pub status: String,
    pub created_at: DateTime<Utc>,
}
