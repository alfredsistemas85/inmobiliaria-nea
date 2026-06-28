use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use super::models::{PaymentProviderAccount, PaymentProviderTransaction};

pub struct ProviderRepository;

impl ProviderRepository {
    pub async fn get_credentials(
        pool: &PgPool,
        tenant_id: Uuid,
        provider: &str,
    ) -> Result<Option<PaymentProviderAccount>, sqlx::Error> {
        sqlx::query_as::<_, PaymentProviderAccount>(
            r#"
            SELECT * FROM payment_provider_accounts
            WHERE tenant_id = $1 AND provider = $2 AND status = 'ACTIVE'
            "#
        )
        .bind(tenant_id)
        .bind(provider)
        .fetch_optional(pool)
        .await
    }

    pub async fn insert_transaction(
        tx: &mut Transaction<'_, Postgres>,
        transaction: &PaymentProviderTransaction,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO payment_provider_transactions (
                id, tenant_id, provider, external_id, payment_id, status, payload, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#
        )
        .bind(transaction.id)
        .bind(transaction.tenant_id)
        .bind(&transaction.provider)
        .bind(&transaction.external_id)
        .bind(transaction.payment_id)
        .bind(&transaction.status)
        .bind(&transaction.payload)
        .bind(transaction.created_at)
        .bind(transaction.updated_at)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn update_transaction_status(
        tx: &mut Transaction<'_, Postgres>,
        transaction_id: Uuid,
        payment_id: Option<Uuid>,
        status: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE payment_provider_transactions
            SET status = $1, payment_id = $2, updated_at = NOW()
            WHERE id = $3
            "#
        )
        .bind(status)
        .bind(payment_id)
        .bind(transaction_id)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}
