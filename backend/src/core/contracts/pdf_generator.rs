use async_trait::async_trait;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use uuid::Uuid;

pub struct CertificateData {
    pub tenant_id: Uuid,
    pub contract_id: Uuid,
    pub rent_adjustment_id: Uuid,
    pub real_estate_name: String,
    pub owner_name: String,
    pub tenant_name: String,
    pub property_address: String,
    pub previous_amount: Decimal,
    pub new_amount: Decimal,
    pub method: String,
    pub percentage: Decimal,
    pub effective_date: NaiveDate,
    pub approver_name: String,
    pub issue_date: NaiveDate,
}

#[async_trait]
pub trait PdfGenerator: Send + Sync {
    async fn generate_adjustment_certificate(
        &self,
        data: CertificateData,
    ) -> Result<Vec<u8>, String>;

    async fn generate_legal_contract(
        &self,
        contract_data: serde_json::Value,
    ) -> Result<Vec<u8>, String>;
}
