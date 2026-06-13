use sqlx::PgPool;
use uuid::Uuid;
use crate::models::property::Property;
use std::sync::Arc;

pub struct PropertyRepository {
    pool: Arc<PgPool>,
}

impl PropertyRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, id: Uuid, tenant_id: Uuid) -> Result<Option<Property>, sqlx::Error> {
        sqlx::query_as::<_, Property>(
            r#"SELECT id, tenant_id, title, description, property_type, operation_type, price, currency, address, city, province, square_meters, bedrooms, bathrooms, status, features, created_at, updated_at 
               FROM properties WHERE id = $1 AND tenant_id = $2"#
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
               RETURNING id, tenant_id, title, description, property_type, operation_type, price, currency, address, city, province, square_meters, bedrooms, bathrooms, status, features, created_at, updated_at"#
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
}
