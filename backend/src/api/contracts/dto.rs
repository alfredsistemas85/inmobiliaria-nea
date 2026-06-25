use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::Deserialize;
use uuid::Uuid;
use super::models::{AdjustmentMethod, AdjustmentFrequency, AutomationMode};

#[derive(Deserialize)]
pub struct CreateContractDto {
    pub property_id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub original_rent_amount: Decimal,
    pub adjustment_method: Option<AdjustmentMethod>,
    pub adjustment_frequency: Option<AdjustmentFrequency>,
    pub automation_mode: Option<AutomationMode>,
    pub fixed_percentage: Option<Decimal>,
    pub notification_days_before: Option<i32>,
}

#[derive(Deserialize)]
pub struct ProposeAdjustmentDto {
    // Si queremos forzar manual en la propuesta, podemos enviar un target_date o usar defaults
}

#[derive(Deserialize)]
pub struct ApproveAdjustmentDto {
    pub new_amount: Option<Decimal>, // Permite modificar el valor antes de aprobar; null = usar monto calculado
    pub notes: Option<String>,
}

#[derive(Deserialize)]
pub struct RollbackAdjustmentDto {
    pub reason: String,
}
