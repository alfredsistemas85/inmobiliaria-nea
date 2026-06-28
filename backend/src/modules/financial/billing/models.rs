use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Installment {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub contract_account_id: Uuid,
    pub number: i32,
    pub due_date: NaiveDate,
    pub original_amount: Decimal,
    pub current_amount: Decimal,
    pub interest_amount: Decimal,
    pub paid_amount: Decimal,
    pub remaining_balance: Decimal,
    pub currency: String,
    pub index_value: Option<Decimal>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
