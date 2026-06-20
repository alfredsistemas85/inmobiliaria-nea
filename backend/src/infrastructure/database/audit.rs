use serde_json::Value;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

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
            r#"INSERT INTO audit_logs (tenant_id, user_id, action, entity_type, entity_id, new_data)
               VALUES ($1, $2, $3, $4, $5, $6)"#,
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

    #[allow(clippy::too_many_arguments)]
    pub async fn log_full(
        &self,
        tenant_id: Option<Uuid>,
        user_id: Option<Uuid>,
        action: &str,
        entity_type: &str,
        entity_id: Option<Uuid>,
        old_data: Option<Value>,
        new_data: Option<Value>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"INSERT INTO audit_logs (tenant_id, user_id, action, entity_type, entity_id, old_data, new_data, ip_address, user_agent)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"#,
        )
        .bind(tenant_id)
        .bind(user_id)
        .bind(action)
        .bind(entity_type)
        .bind(entity_id)
        .bind(old_data)
        .bind(new_data)
        .bind(ip_address)
        .bind(user_agent)
        .execute(&*self.pool)
        .await?;

        Ok(())
    }
}
