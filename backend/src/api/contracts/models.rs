use chrono::{NaiveDate, DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "adjustment_method", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AdjustmentMethod {
    Manual,
    FixedPercentage,
    Ipc,
    Icl,
    CasaPropia,
    Custom,
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "adjustment_frequency", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AdjustmentFrequency {
    Monthly,
    Bimonthly,
    Quarterly,
    Semester,
    Annual,
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "adjustment_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AdjustmentStatus {
    Pending,
    Approved,
    Rejected,
    RolledBack,
    PendingIndexData,
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "automation_mode", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AutomationMode {
    Manual,
    Semiautomatic,
    Automatic,
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "installment_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InstallmentStatus {
    Pending,
    PartiallyPaid,
    Paid,
    Overdue,
    Cancelled,
}

#[derive(Serialize, sqlx::FromRow, Clone)]
pub struct Contract {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub property_id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub original_rent_amount: Decimal,
    pub current_rent_amount: Option<Decimal>,
    pub adjustment_method: Option<AdjustmentMethod>,
    pub adjustment_frequency: Option<AdjustmentFrequency>,
    pub automation_mode: Option<AutomationMode>,
    pub fixed_percentage: Option<Decimal>,
    pub first_notification_days: Option<i32>,
    pub second_notification_days: Option<i32>,
    pub third_notification_days: Option<i32>,
    pub requires_manual_approval: Option<bool>,
    pub next_adjustment_date: Option<NaiveDate>,
    pub last_adjustment_date: Option<NaiveDate>,
    pub status: Option<String>,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct RentAdjustment {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub contract_id: Uuid,
    pub adjustment_method: AdjustmentMethod,
    pub status: AdjustmentStatus,
    pub previous_amount: Decimal,
    pub new_amount: Decimal,
    pub percentage_applied: Option<Decimal>,
    pub index_name: Option<String>,
    pub index_initial_value: Option<Decimal>,
    pub index_final_value: Option<Decimal>,
    pub index_snapshot: Option<serde_json::Value>,
    pub rollback_reason: Option<String>,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub effective_date: NaiveDate,
    pub notes: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct ContractInstallment {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub contract_id: Uuid,
    pub due_date: NaiveDate,
    pub amount: Decimal,
    pub status: InstallmentStatus,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
