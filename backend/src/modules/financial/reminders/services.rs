use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use super::{
    models::NotificationQueue,
    repositories::ReminderRepository,
};

pub struct ReminderService {
    pool: PgPool,
}

impl ReminderService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn schedule_reminder(
        &self,
        tenant_id: Uuid,
        installment_id: Option<Uuid>,
        channel: &str,
        scheduled_at: DateTime<Utc>,
    ) -> Result<Uuid, String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let notification = NotificationQueue {
            id: Uuid::new_v4(),
            tenant_id,
            installment_id,
            channel: channel.to_string(),
            scheduled_at,
            sent_at: None,
            status: "PENDING".to_string(),
            created_at: Utc::now(),
        };

        ReminderRepository::enqueue_notification(&mut tx, &notification)
            .await
            .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(notification.id)
    }

    /// Background worker function to process due reminders
    pub async fn process_queue(&self) -> Result<usize, String> {
        let pending = ReminderRepository::get_pending_notifications(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let count = pending.len();

        for notification in pending {
            // Logic to dispatch message via channel (Email, WhatsApp)
            // if notification.channel == "WHATSAPP" { ... }

            // Mark as sent
            let _ = ReminderRepository::mark_sent(&self.pool, notification.id).await;
        }

        Ok(count)
    }
}
