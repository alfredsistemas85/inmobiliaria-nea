use crate::models::common::PaginatedResponse;
use crate::models::lead::{Lead, LeadActivity};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct LeadRepository {
    pool: Arc<PgPool>,
}

impl LeadRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        tenant_id: Uuid,
        client_id: Uuid,
        property_id: Option<Uuid>,
        status: Option<&str>,
        source: Option<&str>,
        assigned_to: Option<Uuid>,
    ) -> Result<Lead, sqlx::Error> {
        let lead = sqlx::query_as::<_, Lead>(
            r#"
            INSERT INTO leads (tenant_id, client_id, property_id, status, source, assigned_to)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, tenant_id, client_id, property_id, status, source, assigned_to, created_at, updated_at, deleted_at
            "#
        )
        .bind(tenant_id)
        .bind(client_id)
        .bind(property_id)
        .bind(status)
        .bind(source)
        .bind(assigned_to)
        .fetch_one(&*self.pool)
        .await?;

        Ok(lead)
    }

    pub async fn list(
        &self,
        tenant_id: Uuid,
        limit: i64,
        offset: i64,
        q: Option<&str>,
    ) -> Result<PaginatedResponse<Lead>, sqlx::Error> {
        let q_pattern = q.map(|s| format!("%{}%", s));

        let total: (i64,) = if let Some(ref query_str) = q_pattern {
            sqlx::query_as(
                "SELECT COUNT(*) FROM leads l
                 LEFT JOIN clients c ON l.client_id = c.id
                 WHERE l.tenant_id = $1 AND l.deleted_at IS NULL AND (c.first_name ILIKE $2 OR c.last_name ILIKE $2)"
            )
            .bind(tenant_id)
            .bind(query_str)
            .fetch_one(&*self.pool)
            .await?
        } else {
            sqlx::query_as("SELECT COUNT(*) FROM leads WHERE tenant_id = $1 AND deleted_at IS NULL")
                .bind(tenant_id)
                .fetch_one(&*self.pool)
                .await?
        };

        let leads = if let Some(ref query_str) = q_pattern {
            sqlx::query_as::<_, Lead>(
                r#"
                SELECT l.id, l.tenant_id, l.client_id, l.property_id, l.status, l.source, l.assigned_to, l.created_at, l.updated_at, l.deleted_at 
                FROM leads l
                LEFT JOIN clients c ON l.client_id = c.id
                WHERE l.tenant_id = $1 AND l.deleted_at IS NULL AND (c.first_name ILIKE $2 OR c.last_name ILIKE $2)
                ORDER BY l.created_at DESC LIMIT $3 OFFSET $4
                "#
            )
            .bind(tenant_id)
            .bind(query_str)
            .bind(limit)
            .bind(offset)
            .fetch_all(&*self.pool)
            .await?
        } else {
            sqlx::query_as::<_, Lead>(
                r#"
                SELECT id, tenant_id, client_id, property_id, status, source, assigned_to, created_at, updated_at, deleted_at 
                FROM leads 
                WHERE tenant_id = $1 AND deleted_at IS NULL
                ORDER BY created_at DESC LIMIT $2 OFFSET $3
                "#
            )
            .bind(tenant_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&*self.pool)
            .await?
        };

        Ok(PaginatedResponse {
            data: leads,
            total: total.0,
            limit,
            offset,
        })
    }

    pub async fn get_by_id(&self, id: Uuid, tenant_id: Uuid) -> Result<Option<Lead>, sqlx::Error> {
        let lead = sqlx::query_as::<_, Lead>(
            "SELECT id, tenant_id, client_id, property_id, status, source, assigned_to, created_at, updated_at, deleted_at FROM leads WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL"
        )
        .bind(id)
        .bind(tenant_id)
        .fetch_optional(&*self.pool)
        .await?;

        Ok(lead)
    }

    pub async fn update(
        &self,
        id: Uuid,
        tenant_id: Uuid,
        status: Option<&str>,
        assigned_to: Option<Uuid>,
        source: Option<&str>,
    ) -> Result<Lead, sqlx::Error> {
        let lead = sqlx::query_as::<_, Lead>(
            r#"
            UPDATE leads
            SET status = COALESCE($3, status),
                assigned_to = COALESCE($4, assigned_to),
                source = COALESCE($5, source),
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL
            RETURNING id, tenant_id, client_id, property_id, status, source, assigned_to, created_at, updated_at, deleted_at
            "#
        )
        .bind(id)
        .bind(tenant_id)
        .bind(status)
        .bind(assigned_to)
        .bind(source)
        .fetch_one(&*self.pool)
        .await?;

        Ok(lead)
    }

    pub async fn soft_delete(&self, id: Uuid, tenant_id: Uuid) -> Result<u64, sqlx::Error> {
        let result: sqlx::postgres::PgQueryResult = sqlx::query!(
            "UPDATE leads SET deleted_at = CURRENT_TIMESTAMP WHERE id = $1 AND tenant_id = $2",
            id,
            tenant_id
        )
        .execute(&*self.pool)
        .await?;

        Ok(result.rows_affected())
    }

    pub async fn log_activity(
        &self,
        tenant_id: Uuid,
        lead_id: Uuid,
        user_id: Option<Uuid>,
        activity_type: &str,
        description: Option<&str>,
    ) -> Result<LeadActivity, sqlx::Error> {
        let activity = sqlx::query_as::<_, LeadActivity>(
            r#"
            INSERT INTO lead_activities (tenant_id, lead_id, user_id, activity_type, description)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tenant_id, lead_id, user_id, activity_type, description, created_at
            "#,
        )
        .bind(tenant_id)
        .bind(lead_id)
        .bind(user_id)
        .bind(activity_type)
        .bind(description)
        .fetch_one(&*self.pool)
        .await?;

        Ok(activity)
    }
}
