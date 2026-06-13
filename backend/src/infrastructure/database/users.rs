use sqlx::PgPool;
use uuid::Uuid;
use crate::models::user::User;
use std::sync::Arc;

pub struct UserRepository {
    pool: Arc<PgPool>,
}

impl UserRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"SELECT id, tenant_id, role_id, email, password_hash, first_name, last_name, is_active, created_at, updated_at 
               FROM users WHERE email = $1"#
        )
        .bind(email)
        .fetch_optional(&*self.pool)
        .await
    }

    pub async fn find_by_id(&self, id: Uuid, tenant_id: Option<Uuid>) -> Result<Option<User>, sqlx::Error> {
        if let Some(t_id) = tenant_id {
            sqlx::query_as::<_, User>(
                r#"SELECT id, tenant_id, role_id, email, password_hash, first_name, last_name, is_active, created_at, updated_at 
                   FROM users WHERE id = $1 AND tenant_id = $2"#
            )
            .bind(id)
            .bind(t_id)
            .fetch_optional(&*self.pool)
            .await
        } else {
            sqlx::query_as::<_, User>(
                r#"SELECT id, tenant_id, role_id, email, password_hash, first_name, last_name, is_active, created_at, updated_at 
                   FROM users WHERE id = $1"#
            )
            .bind(id)
            .fetch_optional(&*self.pool)
            .await
        }
    }

    pub async fn create(&self, user: User) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"INSERT INTO users (id, tenant_id, role_id, email, password_hash, first_name, last_name, is_active)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
               RETURNING id, tenant_id, role_id, email, password_hash, first_name, last_name, is_active, created_at, updated_at"#
        )
        .bind(user.id)
        .bind(user.tenant_id)
        .bind(user.role_id)
        .bind(user.email)
        .bind(user.password_hash)
        .bind(user.first_name)
        .bind(user.last_name)
        .bind(user.is_active)
        .fetch_one(&*self.pool)
        .await
    }
}
