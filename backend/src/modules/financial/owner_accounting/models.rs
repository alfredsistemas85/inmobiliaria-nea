use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct OwnerStatement {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub owner_id: Uuid,
    pub contract_id: Uuid,
    pub period: String,
    pub gross_income: Decimal,
    pub commission_amount: Decimal,
    pub expenses_amount: Decimal,
    pub taxes_amount: Decimal,
    pub net_amount: Decimal,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub approved_at: Option<DateTime<Utc>>,
    pub paid_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct OwnerStatementItem {
    pub id: Uuid,
    pub statement_id: Uuid,
    pub item_type: String,
    pub amount: Decimal,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct OwnerPayment {
    pub id: Uuid,
    pub statement_id: Uuid,
    pub payment_method: String,
    pub amount: Decimal,
    pub payment_date: DateTime<Utc>,
    pub reference: Option<String>,
    pub status: String,
}
