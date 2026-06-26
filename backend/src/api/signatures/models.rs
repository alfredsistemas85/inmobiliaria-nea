use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type, PartialEq)]
#[sqlx(type_name = "signature_type_enum", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SignatureType {
    Handdrawn,
    DigitalCertificate,
    Otp,
    Biometric,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type, PartialEq)]
#[sqlx(
    type_name = "signature_request_status",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum SignatureRequestStatus {
    Pending,
    Opened,
    Viewed,
    Signed,
    Rejected,
    Expired,
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct ContractSignatureRequest {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub contract_id: Uuid,
    pub participant_id: Uuid,
    pub token_hash: String,
    pub verification_code: String,
    pub signature_order: i32,
    pub required_signature: bool,
    pub signature_type: SignatureType,
    pub status: SignatureRequestStatus,
    pub expires_at: Option<DateTime<Utc>>,
    pub opened_at: Option<DateTime<Utc>>,
    pub viewed_at: Option<DateTime<Utc>>,
    pub signed_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub deleted_by: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct ContractSignature {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub contract_id: Uuid,
    pub participant_id: Uuid,
    pub request_id: Uuid,
    pub signature_image_path: Option<String>,
    pub signature_sha256: Option<String>,
    pub pdf_sha256: Option<String>,
    pub signed_pdf_path: Option<String>,
    pub browser: Option<String>,
    pub operating_system: Option<String>,
    pub ip: Option<String>,
    pub user_agent: Option<String>,
    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,
    pub signed_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct ContractSignatureEvent {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub request_id: Uuid,
    pub event_type: String,
    pub description: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: Option<DateTime<Utc>>,
    pub created_by: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct ContractSignatureSession {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub request_id: Uuid,
    pub opened_at: Option<DateTime<Utc>>,
    pub closed_at: Option<DateTime<Utc>>,
    pub browser: Option<String>,
    pub os: Option<String>,
    pub ip: Option<String>,
    pub user_agent: Option<String>,
    pub fingerprint: Option<String>,
    pub duration_seconds: Option<i32>,
    pub attempts: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct ContractSnapshot {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub contract_id: Uuid,
    pub snapshot_json: serde_json::Value,
    pub created_at: Option<DateTime<Utc>>,
}
