use sqlx::PgPool;
use uuid::Uuid;
use crate::models::role::Role;
use std::sync::Arc;

pub struct RoleRepository {
    pool: Arc<PgPool>,
}

impl RoleRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn find_by_name(&self, name: &str) -> Result<Option<Role>, sqlx::Error> {
        sqlx::query_as::<_, Role>("SELECT id, name, description FROM roles WHERE name = $1")
            .bind(name)
            .fetch_optional(&*self.pool)
            .await
    }
}
