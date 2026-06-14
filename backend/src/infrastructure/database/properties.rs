use sqlx::PgPool;
use uuid::Uuid;
use crate::models::{property::Property, common::PaginatedResponse};
use std::sync::Arc;

pub struct PropertyRepository {
    pool: Arc<PgPool>,
}

impl PropertyRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn list(&self, tenant_id: Uuid, limit: i64, offset: i64, q: Option<&str>) -> Result<PaginatedResponse<Property>, sqlx::Error> {
        let q_pattern = q.map(|s| format!("%{}%", s));
        
        let total: (i64,) = if let Some(ref q_str) = q_pattern {
            sqlx::query_as(
                "SELECT COUNT(*) FROM properties WHERE tenant_id = $1 AND deleted_at IS NULL AND (title ILIKE $2 OR address ILIKE $2 OR city ILIKE $2)"
            )
            .bind(tenant_id)
            .bind(q_str)
            .fetch_one(&*self.pool)
            .await?
        } else {
            sqlx::query_as(
                "SELECT COUNT(*) FROM properties WHERE tenant_id = $1 AND deleted_at IS NULL"
            )
            .bind(tenant_id)
            .fetch_one(&*self.pool)
            .await?
        };

        let properties = if let Some(ref q_str) = q_pattern {
            sqlx::query_as::<_, Property>(
                r#"
                SELECT id, tenant_id, title, description, property_type, operation_type, price, currency, address, city, province, square_meters, bedrooms, bathrooms, status, features, created_at, updated_at, deleted_at 
                FROM properties 
                WHERE tenant_id = $1 AND deleted_at IS NULL AND (title ILIKE $2 OR address ILIKE $2 OR city ILIKE $2)
                ORDER BY created_at DESC LIMIT $3 OFFSET $4
                "#
            )
            .bind(tenant_id)
            .bind(q_str)
            .bind(limit)
            .bind(offset)
            .fetch_all(&*self.pool)
            .await?
        } else {
            sqlx::query_as::<_, Property>(
                r#"
                SELECT id, tenant_id, title, description, property_type, operation_type, price, currency, address, city, province, square_meters, bedrooms, bathrooms, status, features, created_at, updated_at, deleted_at 
                FROM properties 
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
            data: properties,
            total: total.0,
            limit,
            offset,
        })
    }

    pub async fn find_by_id(&self, id: Uuid, tenant_id: Uuid) -> Result<Option<Property>, sqlx::Error> {
        sqlx::query_as::<_, Property>(
            r#"SELECT id, tenant_id, title, description, property_type, operation_type, price, currency, address, city, province, square_meters, bedrooms, bathrooms, status, features, created_at, updated_at, deleted_at 
               FROM properties WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL"#
        )
        .bind(id)
        .bind(tenant_id)
        .fetch_optional(&*self.pool)
        .await
    }

    pub async fn create(&self, property: Property) -> Result<Property, sqlx::Error> {
        sqlx::query_as::<_, Property>(
            r#"INSERT INTO properties (id, tenant_id, title, description, property_type, operation_type, price, currency, address, city, province, square_meters, bedrooms, bathrooms, status, features)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
               RETURNING id, tenant_id, title, description, property_type, operation_type, price, currency, address, city, province, square_meters, bedrooms, bathrooms, status, features, created_at, updated_at, deleted_at"#
        )
        .bind(property.id)
        .bind(property.tenant_id)
        .bind(property.title)
        .bind(property.description)
        .bind(property.property_type)
        .bind(property.operation_type)
        .bind(property.price)
        .bind(property.currency)
        .bind(property.address)
        .bind(property.city)
        .bind(property.province)
        .bind(property.square_meters)
        .bind(property.bedrooms)
        .bind(property.bathrooms)
        .bind(property.status)
        .bind(property.features)
        .fetch_one(&*self.pool)
        .await
    }

    pub async fn soft_delete(&self, id: Uuid, tenant_id: Uuid) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            "UPDATE properties SET deleted_at = CURRENT_TIMESTAMP WHERE id = $1 AND tenant_id = $2",
            id,
            tenant_id
        )
        .execute(&*self.pool)
        .await?;

        Ok(result.rows_affected())
    }
}
