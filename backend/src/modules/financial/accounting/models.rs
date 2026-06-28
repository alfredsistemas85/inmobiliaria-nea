use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ChartOfAccount {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub code: String,
    pub name: String,
    pub account_type: String, // ASSET, LIABILITY, EQUITY, REVENUE, EXPENSE
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct JournalEntry {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub entry_date: NaiveDate,
    pub description: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct JournalEntryLine {
    pub id: Uuid,
    pub entry_id: Uuid,
    pub account_id: Uuid,
    pub debit: Decimal,
    pub credit: Decimal,
    pub cost_center: Option<String>,
    pub reference: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AccountingPeriod {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub month: i32,
    pub year: i32,
    pub status: String,
    pub opened_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
}
