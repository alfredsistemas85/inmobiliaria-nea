use crate::models::tenant::Tenant;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct TenantRepository {
    pool: Arc<PgPool>,
}

impl TenantRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Tenant>, sqlx::Error> {
        sqlx::query_as::<_, Tenant>(
            r#"SELECT id, cuit, dni_responsable, first_name, last_name, business_name, address, phone, city, province, is_active, status, slug, created_at, updated_at 
               FROM tenants WHERE id = $1 AND is_active = true AND status = 'ACTIVE'"#
        )
        .bind(id)
        .fetch_optional(&*self.pool)
        .await
    }


    pub async fn find_by_cuit(&self, cuit: &str) -> Result<Option<Tenant>, sqlx::Error> {
        sqlx::query_as::<_, Tenant>(
            r#"SELECT id, cuit, dni_responsable, first_name, last_name, business_name, address, phone, city, province, is_active, status, slug, created_at, updated_at 
               FROM tenants WHERE cuit = $1"#
        )
        .bind(cuit)
        .fetch_optional(&*self.pool)
        .await
    }

    pub async fn create(&self, tenant: Tenant) -> Result<Tenant, sqlx::Error> {
        sqlx::query_as::<_, Tenant>(
            r#"INSERT INTO tenants (id, cuit, dni_responsable, first_name, last_name, business_name, address, phone, city, province, is_active, status, slug)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
               RETURNING id, cuit, dni_responsable, first_name, last_name, business_name, address, phone, city, province, is_active, status, slug, created_at, updated_at"#
        )
        .bind(tenant.id)
        .bind(tenant.cuit)
        .bind(tenant.dni_responsable)
        .bind(tenant.first_name)
        .bind(tenant.last_name)
        .bind(tenant.business_name)
        .bind(tenant.address)
        .bind(tenant.phone)
        .bind(tenant.city)
        .bind(tenant.province)
        .bind(tenant.is_active)
        .bind(tenant.status)
        .bind(tenant.slug)
        .fetch_one(&*self.pool)
        .await
    }

    pub async fn update_status(&self, id: Uuid, status: &str) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"UPDATE tenants SET status = $1 WHERE id = $2"#
        )
        .bind(status)
        .bind(id)
        .execute(&*self.pool)
        .await?;
        Ok(())
    }

}
