use super::models::{
    AdjustmentFrequency, AdjustmentMethod, AutomationMode, ContractDestination, ContractType,
    GuaranteeType, ParticipantRole,
};
use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::Deserialize;
use uuid::Uuid;

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
pub struct ParticipantGuaranteeDto {
    pub guarantee_type: GuaranteeType,
    pub income_amount: Option<Decimal>,
    pub employer: Option<String>,
    pub guarantee_details: Option<String>,
    pub observations: Option<String>,
}

#[derive(Deserialize)]
pub struct ContractParticipantDto {
    pub client_id: Uuid,
    pub p_role: ParticipantRole,
    pub percentage: Option<Decimal>,
    pub is_main: Option<bool>,
    pub display_order: Option<i32>,
    pub observations: Option<String>,
    pub guarantees: Option<Vec<ParticipantGuaranteeDto>>,
}

#[derive(Deserialize)]
pub struct CreateContractDtoV2 {
    pub property_id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub original_rent_amount: Decimal,
    pub adjustment_method: Option<AdjustmentMethod>,
    pub adjustment_frequency: Option<AdjustmentFrequency>,
    pub automation_mode: Option<AutomationMode>,
    pub fixed_percentage: Option<Decimal>,
    pub notification_days_before: Option<i32>,

    // New fields
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
    
    // Phase 2.1
    pub parent_contract_id: Option<Uuid>,
    pub status: Option<String>,

    pub template_id: Option<Uuid>,

    pub participants: Vec<ContractParticipantDto>,
    pub terms: Option<ContractTermsDto>,
    pub clauses: Option<Vec<ClauseDto>>,
}

#[derive(Deserialize)]
pub struct ContractTermsDto {
    pub allows_pets: Option<bool>,
    pub allows_sublease: Option<bool>,
    pub requires_inventory: Option<bool>,
    pub requires_insurance: Option<bool>,
    pub automatic_renewal: Option<bool>,
    pub permitted_activity: Option<String>,
    pub notice_days: Option<i32>,
    pub early_termination_penalty: Option<String>,
    pub observations: Option<String>,
}

#[derive(Deserialize)]
pub struct ClauseDto {
    pub display_order: i32,
    pub title: String,
    pub body: String,
    pub is_mandatory: Option<bool>,
    pub is_editable: Option<bool>,
    pub is_system: Option<bool>,
}

#[derive(Deserialize)]
pub struct CreateContractTemplateDto {
    pub name: String,
    pub description: Option<String>,
    pub c_type: ContractType,
    pub c_destination: ContractDestination,
    pub clauses: Vec<ClauseDto>,
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

// ── TESTS DE DESERIALIZACIÓN ────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    fn check(json: &str, label: &str) {
        match serde_json::from_str::<CreateContractDto>(json) {
            Ok(_) => { /* pass */ }
            Err(e) => panic!("FALLO [{}]: {}\nJSON: {}", label, e, json),
        }
    }

    #[test]
    fn test_payload_icl_semester_semiautomatic() {
        // Payload EXACTO del screenshot (ICL, SEMESTER, SEMIAUTOMATIC, monto 950000)
        check(
            r#"{
            "property_id": "00000000-0000-0000-0000-000000000001",
            "start_date": "2026-06-25",
            "end_date": "2028-06-25",
            "original_rent_amount": 950000,
            "adjustment_method": "ICL",
            "adjustment_frequency": "SEMESTER",
            "automation_mode": "SEMIAUTOMATIC",
            "notification_days_before": 30
        }"#,
            "ICL+SEMESTER+SEMIAUTOMATIC",
        );
    }

    #[test]
    fn test_payload_ipc_quarterly_semiautomatic() {
        check(
            r#"{
            "property_id": "00000000-0000-0000-0000-000000000001",
            "start_date": "2026-06-25",
            "end_date": "2028-06-25",
            "original_rent_amount": 150000,
            "adjustment_method": "IPC",
            "adjustment_frequency": "QUARTERLY",
            "automation_mode": "SEMIAUTOMATIC",
            "notification_days_before": 30
        }"#,
            "IPC+QUARTERLY+SEMIAUTOMATIC",
        );
    }

    #[test]
    fn test_payload_fixed_percentage() {
        check(
            r#"{
            "property_id": "00000000-0000-0000-0000-000000000001",
            "start_date": "2026-06-25",
            "end_date": "2028-06-25",
            "original_rent_amount": 200000,
            "adjustment_method": "FIXED_PERCENTAGE",
            "adjustment_frequency": "ANNUAL",
            "automation_mode": "AUTOMATIC",
            "fixed_percentage": 10.5,
            "notification_days_before": 30
        }"#,
            "FIXED_PERCENTAGE+ANNUAL+AUTOMATIC",
        );
    }

    #[test]
    fn test_payload_null_optionals() {
        check(
            r#"{
            "property_id": "00000000-0000-0000-0000-000000000001",
            "start_date": "2026-06-25",
            "end_date": "2028-06-25",
            "original_rent_amount": 950000,
            "adjustment_method": null,
            "adjustment_frequency": null,
            "automation_mode": null,
            "notification_days_before": null
        }"#,
            "null optionals",
        );
    }

    #[test]
    fn test_enum_all_adjustment_methods() {
        for variant in &[
            "MANUAL",
            "FIXED_PERCENTAGE",
            "IPC",
            "ICL",
            "CASA_PROPIA",
            "CUSTOM",
        ] {
            let json = format!(
                r#"{{"property_id":"00000000-0000-0000-0000-000000000001","start_date":"2026-06-25","end_date":"2028-06-25","original_rent_amount":100,"adjustment_method":"{}"}}"#,
                variant
            );
            check(&json, variant);
        }
    }

    #[test]
    fn test_enum_all_frequencies() {
        for variant in &["MONTHLY", "BIMONTHLY", "QUARTERLY", "SEMESTER", "ANNUAL"] {
            let json = format!(
                r#"{{"property_id":"00000000-0000-0000-0000-000000000001","start_date":"2026-06-25","end_date":"2028-06-25","original_rent_amount":100,"adjustment_frequency":"{}"}}"#,
                variant
            );
            check(&json, variant);
        }
    }

    #[test]
    fn test_enum_all_automation_modes() {
        for variant in &["MANUAL", "SEMIAUTOMATIC", "AUTOMATIC"] {
            let json = format!(
                r#"{{"property_id":"00000000-0000-0000-0000-000000000001","start_date":"2026-06-25","end_date":"2028-06-25","original_rent_amount":100,"automation_mode":"{}"}}"#,
                variant
            );
            check(&json, variant);
        }
    }
}
