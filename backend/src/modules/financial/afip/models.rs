use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ElectronicInvoice {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub receipt_id: Uuid,
    pub invoice_type: String, // e.g. Factura A, Factura C
    pub point_of_sale: i32,
    pub invoice_number: i32,
    pub cae: Option<String>,
    pub cae_expiration: Option<NaiveDate>,
    pub status: String, // PENDING, APPROVED, REJECTED
    pub request_payload: Option<Value>,
    pub response_payload: Option<Value>,
    pub pdf_path: Option<String>,
    pub xml_path: Option<String>,
    pub created_at: DateTime<Utc>,
}
