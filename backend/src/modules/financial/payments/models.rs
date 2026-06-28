use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Payment {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub account_id: Uuid,
    pub receipt_id: Option<Uuid>,
    pub payment_method: String,
    pub payment_reference: Option<String>,
    pub amount: Decimal,
    pub currency: String,
    pub payment_date: DateTime<Utc>,
    pub status: String,
    pub external_provider: Option<String>,
    pub external_reference: Option<String>,
    pub idempotency_key: Option<Uuid>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentAllocation {
    pub id: Uuid,
    pub payment_id: Uuid,
    pub installment_id: Uuid,
    pub principal_amount: Decimal,
    pub interest_amount: Decimal,
    pub expense_amount: Decimal,
    pub total_allocated: Decimal,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Receipt {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub receipt_number: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptItem {
    pub id: Uuid,
    pub receipt_id: Uuid,
    pub description: String,
    pub amount: Decimal,
}
