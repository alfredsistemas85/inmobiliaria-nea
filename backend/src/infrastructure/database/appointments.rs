use crate::models::appointment::Appointment;
use crate::models::common::PaginatedResponse;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct AppointmentRepository {
    pool: Arc<PgPool>,
}

impl AppointmentRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        tenant_id: Uuid,
        client_id: Uuid,
        property_id: Option<Uuid>,
        user_id: Option<Uuid>,
        scheduled_at: DateTime<Utc>,
        status: Option<&str>,
        notes: Option<&str>,
    ) -> Result<Appointment, sqlx::Error> {
        let appointment = sqlx::query_as::<_, Appointment>(
            r#"
            INSERT INTO appointments (tenant_id, client_id, property_id, user_id, scheduled_at, status, notes)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, tenant_id, client_id, property_id, user_id, scheduled_at, status, notes, confirmed_at, cancelled_at, confirmation_sent_at, created_at, updated_at, deleted_at
            "#
        )
        .bind(tenant_id)
        .bind(client_id)
        .bind(property_id)
        .bind(user_id)
        .bind(scheduled_at)
        .bind(status)
        .bind(notes)
        .fetch_one(&*self.pool)
        .await?;

        Ok(appointment)
    }

    pub async fn list(
        &self,
        tenant_id: Uuid,
        limit: i64,
        offset: i64,
        q: Option<&str>,
    ) -> Result<PaginatedResponse<Appointment>, sqlx::Error> {
        let q_pattern = q.map(|s| format!("%{}%", s));

        let total: (i64,) = if let Some(ref query_str) = q_pattern {
            sqlx::query_as(
                "SELECT COUNT(*) FROM appointments a
                 LEFT JOIN clients c ON a.client_id = c.id
                 WHERE a.tenant_id = $1 AND a.deleted_at IS NULL AND (a.notes ILIKE $2 OR c.first_name ILIKE $2 OR c.last_name ILIKE $2)"
            )
            .bind(tenant_id)
            .bind(query_str)
            .fetch_one(&*self.pool)
            .await?
        } else {
            sqlx::query_as(
                "SELECT COUNT(*) FROM appointments WHERE tenant_id = $1 AND deleted_at IS NULL",
            )
            .bind(tenant_id)
            .fetch_one(&*self.pool)
            .await?
        };

        let appointments = if let Some(ref query_str) = q_pattern {
            sqlx::query_as::<_, Appointment>(
                r#"
                SELECT a.id, a.tenant_id, a.client_id, a.property_id, a.user_id, a.scheduled_at, a.status, a.notes, a.confirmed_at, a.cancelled_at, a.confirmation_sent_at, a.created_at, a.updated_at, a.deleted_at 
                FROM appointments a
                LEFT JOIN clients c ON a.client_id = c.id
                WHERE a.tenant_id = $1 AND a.deleted_at IS NULL AND (a.notes ILIKE $2 OR c.first_name ILIKE $2 OR c.last_name ILIKE $2)
                ORDER BY a.scheduled_at ASC LIMIT $3 OFFSET $4
                "#
            )
            .bind(tenant_id)
            .bind(query_str)
            .bind(limit)
            .bind(offset)
            .fetch_all(&*self.pool)
            .await?
        } else {
            sqlx::query_as::<_, Appointment>(
                r#"
                SELECT id, tenant_id, client_id, property_id, user_id, scheduled_at, status, notes, confirmed_at, cancelled_at, confirmation_sent_at, created_at, updated_at, deleted_at 
                FROM appointments 
                WHERE tenant_id = $1 AND deleted_at IS NULL
                ORDER BY scheduled_at ASC LIMIT $2 OFFSET $3
                "#
            )
            .bind(tenant_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&*self.pool)
            .await?
        };

        Ok(PaginatedResponse {
            data: appointments,
            total: total.0,
            limit,
            offset,
        })
    }

    pub async fn get_by_id(
        &self,
        id: Uuid,
        tenant_id: Uuid,
    ) -> Result<Option<Appointment>, sqlx::Error> {
        let appointment = sqlx::query_as::<_, Appointment>(
            "SELECT id, tenant_id, client_id, property_id, user_id, scheduled_at, status, notes, confirmed_at, cancelled_at, confirmation_sent_at, created_at, updated_at, deleted_at FROM appointments WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL"
        )
        .bind(id)
        .bind(tenant_id)
        .fetch_optional(&*self.pool)
        .await?;

        Ok(appointment)
    }

    pub async fn update(
        &self,
        id: Uuid,
        tenant_id: Uuid,
        client_id: Option<Uuid>,
        property_id: Option<Uuid>,
        user_id: Option<Uuid>,
        scheduled_at: Option<DateTime<Utc>>,
        status: Option<&str>,
        notes: Option<&str>,
    ) -> Result<Appointment, sqlx::Error> {
        let appointment = sqlx::query_as::<_, Appointment>(
            r#"
            UPDATE appointments
            SET client_id = COALESCE($3, client_id),
                property_id = COALESCE($4, property_id),
                user_id = COALESCE($5, user_id),
                scheduled_at = COALESCE($6, scheduled_at),
                status = COALESCE($7, status),
                notes = COALESCE($8, notes),
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL
            RETURNING id, tenant_id, client_id, property_id, user_id, scheduled_at, status, notes, confirmed_at, cancelled_at, confirmation_sent_at, created_at, updated_at, deleted_at
            "#
        )
        .bind(id)
        .bind(tenant_id)
        .bind(client_id)
        .bind(property_id)
        .bind(user_id)
        .bind(scheduled_at)
        .bind(status)
        .bind(notes)
        .fetch_one(&*self.pool)
        .await?;

        Ok(appointment)
    }

    pub async fn soft_delete(&self, id: Uuid, tenant_id: Uuid) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            "UPDATE appointments SET deleted_at = CURRENT_TIMESTAMP WHERE id = $1 AND tenant_id = $2",
            id,
            tenant_id
        )
        .execute(&*self.pool)
        .await?;

        Ok(result.rows_affected())
    }
}
