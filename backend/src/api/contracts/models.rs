use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "adjustment_method", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AdjustmentMethod {
    Manual,
    FixedPercentage,
    Ipc,
    Icl,
    CasaPropia,
    Custom,
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::Type)]
#[sqlx(
    type_name = "adjustment_frequency",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
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
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
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

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "contract_status", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContractStatus {
    Draft,
    PendingSignature,
    Signed,
    Active,
    Suspended,
    Finished,
    Terminated,
    Annulled,
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "contract_type", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContractType {
    Housing,
    Commercial,
    Temporary,
    Professional,
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::Type)]
#[sqlx(
    type_name = "contract_destination",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContractDestination {
    Habitational,
    Commercial,
    Mixed,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, sqlx::Type)]
#[sqlx(type_name = "participant_role", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ParticipantRole {
    Landlord,
    Tenant,
    Guarantor,
    Attorney,
    Witness,
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "guarantee_type", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GuaranteeType {
    Property,
    Payslip,
    SuretyBond,
    Bank,
    Mixed,
    Other,
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
    pub status: Option<ContractStatus>,

    // Phase 1 New Fields
    pub contract_number: Option<String>,
    pub c_type: Option<ContractType>,
    pub c_destination: Option<ContractDestination>,
    pub jurisdiction: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub currency: Option<String>,
    pub deposit_amount: Option<Decimal>,
    pub commission_amount: Option<Decimal>,
    pub fees_amount: Option<Decimal>,
    pub taxes_payer: Option<String>,
    pub services_payer: Option<String>,
    pub observations: Option<String>,
    
    // Phase 2.1 New Fields
    pub snapshot_payload: Option<serde_json::Value>,
    pub parent_contract_id: Option<Uuid>,
}

#[derive(Serialize, sqlx::FromRow, Clone)]
pub struct ContractParticipant {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub contract_id: Uuid,
    pub client_id: Uuid,
    pub p_role: ParticipantRole,
    pub percentage: Option<Decimal>,
    pub is_main: Option<bool>,
    pub display_order: Option<i32>,
    pub observations: Option<String>,
}

#[derive(Serialize, sqlx::FromRow, Clone)]
pub struct ParticipantGuarantee {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub participant_id: Uuid,
    pub guarantee_type: GuaranteeType,
    pub status: Option<String>,
    pub income_amount: Option<Decimal>,
    pub employer: Option<String>,
    pub guarantee_details: Option<String>,
    pub observations: Option<String>,
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

// Fase 2 Models

#[derive(Serialize, sqlx::FromRow, Clone)]
pub struct ContractTemplate {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub c_type: ContractType,
    pub c_destination: ContractDestination,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, sqlx::FromRow, Clone)]
pub struct TemplateClause {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub template_id: Uuid,
    pub display_order: i32,
    pub title: String,
    pub body: String,
    pub is_mandatory: bool,
    pub is_editable: bool,
    pub is_system: bool,
}

#[derive(Serialize, sqlx::FromRow, Clone)]
pub struct ContractTerms {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub contract_id: Uuid,
    pub allows_pets: bool,
    pub allows_sublease: bool,
    pub requires_inventory: bool,
    pub requires_insurance: bool,
    pub automatic_renewal: bool,
    pub permitted_activity: Option<String>,
    pub notice_days: Option<i32>,
    pub early_termination_penalty: Option<String>,
    pub observations: Option<String>,
}

#[derive(Serialize, sqlx::FromRow, Clone)]
pub struct ContractClause {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub contract_id: Uuid,
    pub display_order: i32,
    pub title: String,
    pub body: String,
    pub is_mandatory: bool,
    pub is_editable: bool,
    pub is_system: bool,
}
