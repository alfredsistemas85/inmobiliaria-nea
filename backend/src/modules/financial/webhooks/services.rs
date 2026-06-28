use chrono::Utc;
use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;

use super::{
    models::WebhookEvent,
    repositories::WebhookRepository,
};

pub struct WebhookService {
    pool: PgPool,
}

impl WebhookService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Receives a raw webhook payload and stores it idempotently.
    pub async fn receive_webhook(
        &self,
        tenant_id: Uuid,
        provider: &str,
        event_name: &str,
        payload: Value,
    ) -> Result<Uuid, String> {
        let event = WebhookEvent {
            id: Uuid::new_v4(),
            tenant_id,
            provider: provider.to_string(),
            event: event_name.to_string(),
            payload,
            processed: false,
            processed_at: None,
            retries: 0,
            created_at: Utc::now(),
        };

        WebhookRepository::insert_event(&self.pool, &event)
            .await
            .map_err(|e| e.to_string())?;

        Ok(event.id)
    }

    /// Background worker function to process pending webhooks.
    pub async fn process_pending(&self) -> Result<usize, String> {
        let events = WebhookRepository::get_unprocessed_events(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        
        let count = events.len();

        for event in events {
            // Processing logic depends on event.provider and event.event
            // E.g., if provider == "MERCADO_PAGO" && event == "payment.created"
            // we call ProviderService or PaymentService to mark the installment as paid.

            let mut tx = match self.pool.begin().await {
                Ok(t) => t,
                Err(_) => continue,
            };

            // Assuming processing succeeds:
            if let Err(_) = WebhookRepository::mark_processed(&mut tx, event.id).await {
                let _ = WebhookRepository::increment_retry(&self.pool, event.id).await;
                continue;
            }

            let _ = tx.commit().await;
        }

        Ok(count)
    }
}
