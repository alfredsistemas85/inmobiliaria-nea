use chrono::Utc;
use rust_decimal::Decimal;
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

use super::{
    models::PaymentProviderTransaction,
    repositories::ProviderRepository,
};

pub struct ProviderService {
    pool: PgPool,
}

impl ProviderService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Generates a payment link for a specific provider (e.g., Mercado Pago)
    pub async fn generate_payment_link(
        &self,
        tenant_id: Uuid,
        provider_name: &str,
        amount: Decimal,
        description: &str,
    ) -> Result<String, String> {
        // 1. Get credentials
        let credentials = ProviderRepository::get_credentials(&self.pool, tenant_id, provider_name)
            .await
            .map_err(|e| e.to_string())?;

        let _creds = match credentials {
            Some(c) => c,
            None => return Err(format!("No active credentials for provider {}", provider_name)),
        };

        // 2. Mock call to provider (e.g., MercadoPago Preferences API)
        // In a real implementation, we would use reqwest to call the API using the credentials.
        
        let external_id = format!("ext_{}", Uuid::new_v4());
        let payment_link = format!("https://sandbox.mercadopago.com.ar/checkout/v1/redirect?pref_id={}", external_id);

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let transaction = PaymentProviderTransaction {
            id: Uuid::new_v4(),
            tenant_id,
            provider: provider_name.to_string(),
            external_id: external_id.clone(),
            payment_id: None,
            status: "PENDING".to_string(),
            payload: json!({ "amount": amount, "description": description }),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        ProviderRepository::insert_transaction(&mut tx, &transaction)
            .await
            .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(payment_link)
    }

    /// Receives a notification from a provider and updates the transaction
    pub async fn process_notification(
        &self,
        tenant_id: Uuid,
        provider_name: &str,
        external_id: &str,
        status: &str,
        payload: Value,
    ) -> Result<(), String> {
        // Find transaction and update
        // (In a real scenario we'd verify the signature of the webhook and find the transaction ID from the DB)
        
        // This acts as a bridge for the Webhook module.
        Ok(())
    }
}
