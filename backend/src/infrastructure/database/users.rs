use crate::models::user::User;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct UserRepository {
    pool: Arc<PgPool>,
}

impl UserRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"SELECT id, tenant_id, role, email, password_hash, first_name, last_name, is_active, email_verified_at, verification_token, verification_sent_at, email_type, onboarding_token, onboarding_token_expires_at, created_at, updated_at 
               FROM users WHERE email = $1 AND deleted_at IS NULL"#
        )
        .bind(email)
        .fetch_optional(&*self.pool)
        .await
    }

    pub async fn find_by_id(
        &self,
        id: Uuid,
        tenant_id: Option<Uuid>,
    ) -> Result<Option<User>, sqlx::Error> {
        if let Some(t_id) = tenant_id {
            sqlx::query_as::<_, User>(
                r#"SELECT id, tenant_id, role, email, password_hash, first_name, last_name, is_active, email_verified_at, verification_token, verification_sent_at, email_type, onboarding_token, onboarding_token_expires_at, created_at, updated_at 
                   FROM users WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL"#
            )
            .bind(id)
            .bind(t_id)
            .fetch_optional(&*self.pool)
            .await
        } else {
            sqlx::query_as::<_, User>(
                r#"SELECT id, tenant_id, role, email, password_hash, first_name, last_name, is_active, email_verified_at, verification_token, verification_sent_at, email_type, onboarding_token, onboarding_token_expires_at, created_at, updated_at 
                   FROM users WHERE id = $1 AND deleted_at IS NULL"#
            )
            .bind(id)
            .fetch_optional(&*self.pool)
            .await
        }
    }

    pub async fn create(&self, user: User) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"INSERT INTO users (id, tenant_id, role, email, password_hash, first_name, last_name, is_active, email_verified_at, verification_token, verification_sent_at, email_type)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
               RETURNING id, tenant_id, role, email, password_hash, first_name, last_name, is_active, email_verified_at, verification_token, verification_sent_at, email_type, onboarding_token, onboarding_token_expires_at, created_at, updated_at"#
        )
        .bind(user.id)
        .bind(user.tenant_id)
        .bind(user.role)
        .bind(user.email)
        .bind(user.password_hash)
        .bind(user.first_name)
        .bind(user.last_name)
        .bind(user.is_active)
        .bind(user.email_verified_at)
        .bind(user.verification_token)
        .bind(user.verification_sent_at)
        .bind(user.email_type)
        .fetch_one(&*self.pool)
        .await
    }

    pub async fn find_with_role_by_email(
        &self,
        email: &str,
    ) -> Result<Option<User>, sqlx::Error> {
        self.find_by_email(email).await
    }

    pub async fn find_with_role_by_id(
        &self,
        id: Uuid,
        tenant_id: Option<Uuid>,
    ) -> Result<Option<User>, sqlx::Error> {
        self.find_by_id(id, tenant_id).await
    }

    pub async fn update_email_verification(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"UPDATE users 
               SET email_verified_at = CURRENT_TIMESTAMP, verification_token = NULL 
               WHERE id = $1"#
        )
        .bind(id)
        .execute(&*self.pool)
        .await?;
        Ok(())
    }

    pub async fn find_by_verification_token(&self, token: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"SELECT id, tenant_id, role, email, password_hash, first_name, last_name, is_active, email_verified_at, verification_token, verification_sent_at, email_type, onboarding_token, onboarding_token_expires_at, created_at, updated_at 
               FROM users WHERE verification_token = $1 AND deleted_at IS NULL"#
        )
        .bind(token)
        .fetch_optional(&*self.pool)
        .await
    }
}
