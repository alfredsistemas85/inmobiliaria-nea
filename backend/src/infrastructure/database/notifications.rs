use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;
use crate::models::notification::Notification;

#[derive(Clone)]
pub struct NotificationRepository {
    pool: Arc<PgPool>,
}

impl NotificationRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        tenant_id: Uuid,
        user_id: Option<Uuid>,
        r#type: &str,
        title: &str,
        content: &str,
    ) -> Result<Notification, sqlx::Error> {
        sqlx::query_as!(
            Notification,
            r#"
            INSERT INTO notifications (tenant_id, user_id, type, title, content)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tenant_id, user_id, type as "type", title, content, read_at, created_at
            "#,
            tenant_id,
            user_id,
            r#type,
            title,
            content
        )
        .fetch_one(&*self.pool)
        .await
    }

    pub async fn list(
        &self,
        tenant_id: Uuid,
        user_id: Uuid,
        limit: i64,
    ) -> Result<Vec<Notification>, sqlx::Error> {
        sqlx::query_as!(
            Notification,
            r#"
            SELECT id, tenant_id, user_id, type as "type", title, content, read_at, created_at
            FROM notifications
            WHERE tenant_id = $1 AND (user_id = $2 OR user_id IS NULL)
            ORDER BY created_at DESC
            LIMIT $3
            "#,
            tenant_id,
            user_id,
            limit
        )
        .fetch_all(&*self.pool)
        .await
    }

    pub async fn count_unread(
        &self,
        tenant_id: Uuid,
        user_id: Uuid,
    ) -> Result<i64, sqlx::Error> {
        let result: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*)
            FROM notifications
            WHERE tenant_id = $1 AND (user_id = $2 OR user_id IS NULL) AND read_at IS NULL
            "#
        )
        .bind(tenant_id)
        .bind(user_id)
        .fetch_one(&*self.pool)
        .await?;

        Ok(result.0)
    }

    pub async fn mark_as_read(
        &self,
        tenant_id: Uuid,
        notification_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE notifications
            SET read_at = CURRENT_TIMESTAMP
            WHERE id = $1 AND tenant_id = $2 AND (user_id = $3 OR user_id IS NULL)
            "#,
            notification_id,
            tenant_id,
            user_id
        )
        .execute(&*self.pool)
        .await?;

        Ok(())
    }
}
