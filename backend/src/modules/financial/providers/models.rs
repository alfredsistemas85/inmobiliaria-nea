use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PaymentProviderAccount {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub provider: String, // e.g. "MERCADO_PAGO"
    pub credentials: Value,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PaymentProviderTransaction {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub provider: String, // e.g. "MERCADO_PAGO"
    pub external_id: String, // the transaction ID from the provider
    pub payment_id: Option<Uuid>, // our internal payment ID once reconciled
    pub status: String, // PENDING, COMPLETED, FAILED
    pub payload: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
