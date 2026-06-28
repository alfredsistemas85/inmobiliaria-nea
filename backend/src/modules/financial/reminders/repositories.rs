use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use super::models::NotificationQueue;

pub struct ReminderRepository;

impl ReminderRepository {
    pub async fn enqueue_notification(
        tx: &mut Transaction<'_, Postgres>,
        notification: &NotificationQueue,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO notification_queue (
                id, tenant_id, installment_id, channel, scheduled_at, sent_at, status, created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#
        )
        .bind(notification.id)
        .bind(notification.tenant_id)
        .bind(notification.installment_id)
        .bind(&notification.channel)
        .bind(notification.scheduled_at)
        .bind(notification.sent_at)
        .bind(&notification.status)
        .bind(notification.created_at)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn get_pending_notifications(
        pool: &PgPool,
    ) -> Result<Vec<NotificationQueue>, sqlx::Error> {
        sqlx::query_as::<_, NotificationQueue>(
            r#"
            SELECT * FROM notification_queue
            WHERE status = 'PENDING' AND scheduled_at <= NOW()
            ORDER BY scheduled_at ASC
            LIMIT 50
            "#
        )
        .fetch_all(pool)
        .await
    }

    pub async fn mark_sent(
        pool: &PgPool,
        notification_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE notification_queue SET status = 'SENT', sent_at = NOW() WHERE id = $1"
        )
        .bind(notification_id)
        .execute(pool)
        .await?;

        Ok(())
    }
}
