use crate::models::client::Client;
use crate::models::common::PaginatedResponse;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct ClientRepository {
    pool: Arc<PgPool>,
}

impl ClientRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn create(&self, tenant_id: Uuid, first_name: Option<&str>, last_name: Option<&str>, phone: &str, email: Option<&str>, notes: Option<&str>) -> Result<Client, sqlx::Error> {
        let client = sqlx::query_as!(
            Client,
            r#"
            INSERT INTO clients (tenant_id, first_name, last_name, phone, email, notes)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
            tenant_id,
            first_name,
            last_name,
            phone,
            email,
            notes
        )
        .fetch_one(&*self.pool)
        .await?;

        Ok(client)
    }

    pub async fn list(&self, tenant_id: Uuid, limit: i64, offset: i64, q: Option<&str>) -> Result<PaginatedResponse<Client>, sqlx::Error> {
        let q_pattern = q.map(|s| format!("%{}%", s));
        
        let total: (i64,) = if let Some(ref q) = q_pattern {
            sqlx::query_as(
                "SELECT COUNT(*) FROM clients WHERE tenant_id = $1 AND deleted_at IS NULL AND (first_name ILIKE $2 OR last_name ILIKE $2 OR phone ILIKE $2)"
            )
            .bind(tenant_id)
            .bind(q)
            .fetch_one(&*self.pool)
            .await?
        } else {
            sqlx::query_as(
                "SELECT COUNT(*) FROM clients WHERE tenant_id = $1 AND deleted_at IS NULL"
            )
            .bind(tenant_id)
            .fetch_one(&*self.pool)
            .await?
        };

        let clients = if let Some(ref q) = q_pattern {
            sqlx::query_as!(
                Client,
                r#"
                SELECT * FROM clients 
                WHERE tenant_id = $1 AND deleted_at IS NULL AND (first_name ILIKE $2 OR last_name ILIKE $2 OR phone ILIKE $2)
                ORDER BY created_at DESC LIMIT $3 OFFSET $4
                "#,
                tenant_id,
                q,
                limit,
                offset
            )
            .fetch_all(&*self.pool)
            .await?
        } else {
            sqlx::query_as!(
                Client,
                r#"
                SELECT * FROM clients 
                WHERE tenant_id = $1 AND deleted_at IS NULL
                ORDER BY created_at DESC LIMIT $2 OFFSET $3
                "#,
                tenant_id,
                limit,
                offset
            )
            .fetch_all(&*self.pool)
            .await?
        };

        Ok(PaginatedResponse {
            data: clients,
            total: total.0,
            limit,
            offset,
        })
    }

    pub async fn get_by_id(&self, id: Uuid, tenant_id: Uuid) -> Result<Option<Client>, sqlx::Error> {
        let client = sqlx::query_as!(
            Client,
            "SELECT * FROM clients WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL",
            id,
            tenant_id
        )
        .fetch_optional(&*self.pool)
        .await?;

        Ok(client)
    }

    pub async fn update(&self, id: Uuid, tenant_id: Uuid, first_name: Option<&str>, last_name: Option<&str>, phone: Option<&str>, email: Option<&str>, notes: Option<&str>) -> Result<Client, sqlx::Error> {
        let client = sqlx::query_as!(
            Client,
            r#"
            UPDATE clients
            SET first_name = COALESCE($3, first_name),
                last_name = COALESCE($4, last_name),
                phone = COALESCE($5, phone),
                email = COALESCE($6, email),
                notes = COALESCE($7, notes),
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL
            RETURNING *
            "#,
            id,
            tenant_id,
            first_name,
            last_name,
            phone,
            email,
            notes
        )
        .fetch_one(&*self.pool)
        .await?;

        Ok(client)
    }

    pub async fn soft_delete(&self, id: Uuid, tenant_id: Uuid) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            "UPDATE clients SET deleted_at = CURRENT_TIMESTAMP WHERE id = $1 AND tenant_id = $2",
            id,
            tenant_id
        )
        .execute(&*self.pool)
        .await?;

        Ok(result.rows_affected())
    }
}
