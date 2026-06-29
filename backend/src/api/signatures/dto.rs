use crate::api::signatures::models::{SignatureRequestStatus, SignatureType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateSignatureRequestDto {
    pub participant_id: Uuid,
    pub signature_order: i32,
    pub required_signature: bool,
    pub signature_type: Option<SignatureType>,
}

#[derive(Debug, Deserialize)]
pub struct InitSignaturesDto {
    pub requests: Vec<CreateSignatureRequestDto>,
}

#[derive(Debug, Serialize)]
pub struct SignatureRequestResponseDto {
    pub id: Uuid,
    pub contract_id: Uuid,
    pub participant_id: Uuid,
    pub status: SignatureRequestStatus,
    pub signature_order: i32,
    pub required_signature: bool,
    pub expires_at: Option<DateTime<Utc>>,
    pub link: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PublicSignatureInfoDto {
    pub contract_id: Uuid,
    pub participant_id: Uuid,
    pub status: SignatureRequestStatus,
    pub expires_at: Option<DateTime<Utc>>,
    pub contract_snapshot: Option<serde_json::Value>, 
    // The public view needs to see what they are signing.
    // However, the snapshot JSON contains all the info (clauses, terms).
}

#[derive(Debug, Deserialize)]
pub struct SubmitSignatureDto {
    // We expect the image data to be uploaded to storage BEFORE calling this endpoint, 
    // or we expect a base64 from the canvas that the backend will upload to storage?
    // The user spec says: "NO guardar Base64 en DB. Subir inmediatamente: firma. Guardar únicamente storage_path".
    // So the frontend sends the base64 to the backend, and the backend uploads it to Storage inside the Transaction.
    pub signature_base64: String, 
    pub browser: Option<String>,
    pub operating_system: Option<String>,
    pub ip: Option<String>,
    pub user_agent: Option<String>,
    pub latitude: Option<rust_decimal::Decimal>,
    pub longitude: Option<rust_decimal::Decimal>,
}

#[derive(Debug, Serialize)]
pub struct SubmitSignatureResponseDto {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct VerificationResponseDto {
    pub valid: bool,
    pub contract_id: Uuid,
    pub signature_sha256: Option<String>,
    pub pdf_sha256: Option<String>,
    pub signed_at: Option<DateTime<Utc>>,
    pub status: SignatureRequestStatus,
    pub participants_signed: usize,
    pub total_participants: usize,
}
