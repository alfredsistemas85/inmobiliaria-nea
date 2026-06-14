use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;
use serde_json::Value;

pub struct AuditRepository {
    pool: Arc<PgPool>,
}

impl AuditRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn log(
        &self,
        tenant_id: Option<Uuid>,
        user_id: Option<Uuid>,
        action: &str,
        entity_type: &str,
        entity_id: Option<Uuid>,
        details: Option<Value>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"INSERT INTO audit_logs (tenant_id, user_id, action, entity_type, entity_id, details)
               VALUES ($1, $2, $3, $4, $5, $6)"#
        )
        .bind(tenant_id)
        .bind(user_id)
        .bind(action)
        .bind(entity_type)
        .bind(entity_id)
        .bind(details)
        .execute(&*self.pool)
        .await?;

        Ok(())
    }
}
