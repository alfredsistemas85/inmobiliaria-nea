use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct BankTransaction {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub account_number: String,
    pub transaction_date: DateTime<Utc>,
    pub description: String,
    pub amount: Decimal,
    pub reference: Option<String>,
    pub reconciliation_status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Reconciliation {
    pub id: Uuid,
    pub payment_id: Option<Uuid>,
    pub bank_transaction_id: Option<Uuid>,
    pub confidence: Decimal,
    pub matched_by: Option<Uuid>,
    pub matched_at: DateTime<Utc>,
}
