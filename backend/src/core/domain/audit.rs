use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AuditLogEntry {
    pub tenant_id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub action: String,
    pub old_payload: Option<Value>,
    pub new_payload: Option<Value>,
    pub actor_id: Uuid,
    pub ip_address: Option<String>,
}

pub struct AuditService {
    pool: PgPool,
}

impl AuditService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Guarda un registro de auditoría en la tabla `system_audit_logs`.
    /// Puede ejecutarse dentro de una transacción proveyendo el executor.
    #[tracing::instrument(skip(self, executor, entry), fields(entity_id = %entry.entity_id, action = %entry.action))]
    pub async fn log_event<'a, E>(&self, executor: E, entry: AuditLogEntry) -> Result<(), sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        sqlx::query!(
            r#"
            INSERT INTO system_audit_logs 
            (tenant_id, entity_type, entity_id, action, old_payload, new_payload, actor_id, ip_address)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            entry.tenant_id,
            entry.entity_type,
            entry.entity_id,
            entry.action,
            entry.old_payload,
            entry.new_payload,
            entry.actor_id,
            entry.ip_address
        )
        .execute(executor)
        .await?;

        Ok(())
    }
}
