use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use super::{
    models::AutopaySubscription,
    repositories::AutopayRepository,
};

pub struct AutopayService {
    pool: PgPool,
}

impl AutopayService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_subscription(
        &self,
        tenant_id: Uuid,
        customer_id: Uuid,
        provider: &str,
        token: &str,
    ) -> Result<Uuid, String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let subscription = AutopaySubscription {
            id: Uuid::new_v4(),
            tenant_id,
            customer_id,
            provider: provider.to_string(),
            token: token.to_string(),
            status: "ACTIVE".to_string(),
            created_at: Utc::now(),
        };

        AutopayRepository::insert_subscription(&mut tx, &subscription)
            .await
            .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(subscription.id)
    }

    pub async fn charge_subscription(
        &self,
        tenant_id: Uuid,
        customer_id: Uuid,
        amount: rust_decimal::Decimal,
    ) -> Result<(), String> {
        let subscription = AutopayRepository::get_active_subscription(&self.pool, tenant_id, customer_id)
            .await
            .map_err(|e| e.to_string())?;

        if let Some(sub) = subscription {
            // Logic to call the payment provider with sub.token to charge the amount
            println!("Charging {} using provider {} with token {}", amount, sub.provider, sub.token);
            Ok(())
        } else {
            Err("No active autopay subscription found".to_string())
        }
    }
}
