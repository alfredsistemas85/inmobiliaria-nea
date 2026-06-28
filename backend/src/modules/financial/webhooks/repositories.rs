use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use super::models::WebhookEvent;

pub struct WebhookRepository;

impl WebhookRepository {
    pub async fn insert_event(
        pool: &PgPool,
        event: &WebhookEvent,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO webhook_events (
                id, tenant_id, provider, event, payload, processed, processed_at, retries, created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#
        )
        .bind(event.id)
        .bind(event.tenant_id)
        .bind(&event.provider)
        .bind(&event.event)
        .bind(&event.payload)
        .bind(event.processed)
        .bind(event.processed_at)
        .bind(event.retries)
        .bind(event.created_at)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get_unprocessed_events(
        pool: &PgPool,
    ) -> Result<Vec<WebhookEvent>, sqlx::Error> {
        sqlx::query_as::<_, WebhookEvent>(
            r#"
            SELECT * FROM webhook_events
            WHERE processed = FALSE AND retries < 5
            ORDER BY created_at ASC
            LIMIT 50
            "#
        )
        .fetch_all(pool)
        .await
    }

    pub async fn mark_processed(
        tx: &mut Transaction<'_, Postgres>,
        event_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE webhook_events
            SET processed = TRUE, processed_at = NOW()
            WHERE id = $1
            "#
        )
        .bind(event_id)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn increment_retry(
        pool: &PgPool,
        event_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE webhook_events SET retries = retries + 1 WHERE id = $1"
        )
        .bind(event_id)
        .execute(pool)
        .await?;

        Ok(())
    }
}
