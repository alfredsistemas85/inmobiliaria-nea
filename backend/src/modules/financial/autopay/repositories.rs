use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use super::models::AutopaySubscription;

pub struct AutopayRepository;

impl AutopayRepository {
    pub async fn insert_subscription(
        tx: &mut Transaction<'_, Postgres>,
        subscription: &AutopaySubscription,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO autopay_subscriptions (id, tenant_id, customer_id, provider, token, status, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#
        )
        .bind(subscription.id)
        .bind(subscription.tenant_id)
        .bind(subscription.customer_id)
        .bind(&subscription.provider)
        .bind(&subscription.token)
        .bind(&subscription.status)
        .bind(subscription.created_at)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn get_active_subscription(
        pool: &PgPool,
        tenant_id: Uuid,
        customer_id: Uuid,
    ) -> Result<Option<AutopaySubscription>, sqlx::Error> {
        sqlx::query_as::<_, AutopaySubscription>(
            "SELECT * FROM autopay_subscriptions WHERE tenant_id = $1 AND customer_id = $2 AND status = 'ACTIVE'"
        )
        .bind(tenant_id)
        .bind(customer_id)
        .fetch_optional(pool)
        .await
    }
}
