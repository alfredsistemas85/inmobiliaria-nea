use crate::models::common::PaginatedResponse;
use crate::models::property::{Property, PropertyDocument, PropertyImage};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct PublicPropertyRepository {
    pool: Arc<PgPool>,
}

impl PublicPropertyRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn list(
        &self,
        tenant_id: Uuid,
        limit: i64,
        offset: i64,
        operation_type: Option<&str>,
        property_type: Option<&str>,
        city: Option<&str>,
        price_min: Option<rust_decimal::Decimal>,
        price_max: Option<rust_decimal::Decimal>,
        bedrooms: Option<i32>,
    ) -> Result<PaginatedResponse<Property>, sqlx::Error> {
        // Removed unused strings

        // Dynamic binding in sqlx is complex without query builder.
        // We will build a raw string and use query_as.

        // Dynamic binding in sqlx is complex without query builder.
        // We will build a raw string and use query_as.

        // This is a naive implementation. For production with variable filters,
        // a QueryBuilder is better, but since it's just a few filters, we can just use simple logic
        // or sqlx QueryBuilder.

        let mut query_builder_count =
            sqlx::QueryBuilder::new("SELECT COUNT(*) FROM properties WHERE tenant_id = ");
        query_builder_count.push_bind(tenant_id);
        query_builder_count.push(" AND deleted_at IS NULL AND status = 'DISPONIBLE'");

        let mut query_builder_select = sqlx::QueryBuilder::new("SELECT id, tenant_id, title, description, property_type, operation_type, price, currency, address, city, province, square_meters, bedrooms, bathrooms, status, features, created_at, updated_at, deleted_at FROM properties WHERE tenant_id = ");
        query_builder_select.push_bind(tenant_id);
        query_builder_select.push(" AND deleted_at IS NULL AND status = 'DISPONIBLE'");

        if let Some(op) = operation_type {
            query_builder_count.push(" AND operation_type = ");
            query_builder_count.push_bind(op);
            query_builder_select.push(" AND operation_type = ");
            query_builder_select.push_bind(op);
        }

        if let Some(pt) = property_type {
            query_builder_count.push(" AND property_type = ");
            query_builder_count.push_bind(pt);
            query_builder_select.push(" AND property_type = ");
            query_builder_select.push_bind(pt);
        }

        if let Some(c) = city {
            query_builder_count.push(" AND city ILIKE ");
            query_builder_count.push_bind(format!("%{}%", c));
            query_builder_select.push(" AND city ILIKE ");
            query_builder_select.push_bind(format!("%{}%", c));
        }

        if let Some(pmin) = price_min {
            query_builder_count.push(" AND price >= ");
            query_builder_count.push_bind(pmin);
            query_builder_select.push(" AND price >= ");
            query_builder_select.push_bind(pmin);
        }

        if let Some(pmax) = price_max {
            query_builder_count.push(" AND price <= ");
            query_builder_count.push_bind(pmax);
            query_builder_select.push(" AND price <= ");
            query_builder_select.push_bind(pmax);
        }

        if let Some(bed) = bedrooms {
            query_builder_count.push(" AND bedrooms >= ");
            query_builder_count.push_bind(bed);
            query_builder_select.push(" AND bedrooms >= ");
            query_builder_select.push_bind(bed);
        }

        let total: (i64,) = query_builder_count
            .build_query_as()
            .fetch_one(&*self.pool)
            .await?;

        query_builder_select.push(" ORDER BY created_at DESC LIMIT ");
        query_builder_select.push_bind(limit);
        query_builder_select.push(" OFFSET ");
        query_builder_select.push_bind(offset);

        let properties = query_builder_select
            .build_query_as::<Property>()
            .fetch_all(&*self.pool)
            .await?;

        Ok(PaginatedResponse {
            data: properties,
            total: total.0,
            limit,
            offset,
        })
    }

    pub async fn get_main_image(
        &self,
        property_id: Uuid,
        tenant_id: Uuid,
    ) -> Result<Option<String>, sqlx::Error> {
        let row: Option<(String,)> = sqlx::query_as("SELECT url FROM property_images WHERE property_id = $1 AND tenant_id = $2 AND is_main = true LIMIT 1")
            .bind(property_id)
            .bind(tenant_id)
            .fetch_optional(&*self.pool)
            .await?;

        Ok(row.map(|r| r.0))
    }

    pub async fn get_all_images(
        &self,
        property_id: Uuid,
        tenant_id: Uuid,
    ) -> Result<Vec<PropertyImage>, sqlx::Error> {
        sqlx::query_as::<_, PropertyImage>("SELECT id, tenant_id, property_id, url, is_main, created_at FROM property_images WHERE property_id = $1 AND tenant_id = $2 ORDER BY is_main DESC, created_at ASC")
            .bind(property_id)
            .bind(tenant_id)
            .fetch_all(&*self.pool)
            .await
    }

    pub async fn get_documents(
        &self,
        property_id: Uuid,
        tenant_id: Uuid,
    ) -> Result<Vec<PropertyDocument>, sqlx::Error> {
        sqlx::query_as::<_, PropertyDocument>("SELECT id, tenant_id, property_id, url, title, created_at FROM property_documents WHERE property_id = $1 AND tenant_id = $2")
            .bind(property_id)
            .bind(tenant_id)
            .fetch_all(&*self.pool)
            .await
    }

    pub async fn get_featured(
        &self,
        tenant_id: Uuid,
        limit: i64,
    ) -> Result<Vec<Property>, sqlx::Error> {
        sqlx::query_as::<_, Property>(
            "SELECT id, tenant_id, title, description, property_type, operation_type, price, currency, address, city, province, square_meters, bedrooms, bathrooms, status, features, created_at, updated_at, deleted_at FROM properties WHERE tenant_id = $1 AND deleted_at IS NULL AND status = 'DISPONIBLE' ORDER BY created_at DESC LIMIT $2"
        )
        .bind(tenant_id)
        .bind(limit)
        .fetch_all(&*self.pool)
        .await
    }
}
