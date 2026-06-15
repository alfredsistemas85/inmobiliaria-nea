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
            r#"SELECT id, tenant_id, role_id, email, password_hash, first_name, last_name, is_active, created_at, updated_at 
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
                r#"SELECT id, tenant_id, role_id, email, password_hash, first_name, last_name, is_active, created_at, updated_at 
                   FROM users WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL"#
            )
            .bind(id)
            .bind(t_id)
            .fetch_optional(&*self.pool)
            .await
        } else {
            sqlx::query_as::<_, User>(
                r#"SELECT id, tenant_id, role_id, email, password_hash, first_name, last_name, is_active, created_at, updated_at 
                   FROM users WHERE id = $1 AND deleted_at IS NULL"#
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

    pub async fn find_with_role_by_email(
        &self,
        email: &str,
    ) -> Result<Option<(User, String)>, sqlx::Error> {
        let row = sqlx::query(
            r#"SELECT u.id, u.tenant_id, u.role_id, u.email, u.password_hash, u.first_name, u.last_name, u.is_active, u.created_at, u.updated_at, r.name as role_name
               FROM users u LEFT JOIN roles r ON u.role_id = r.id WHERE u.email = $1 AND u.deleted_at IS NULL"#
        )
        .bind(email)
        .fetch_optional(&*self.pool)
        .await?;

        if let Some(r) = row {
            use sqlx::Row;
            let user = User {
                id: r.try_get("id")?,
                tenant_id: r.try_get("tenant_id")?,
                role_id: r.try_get("role_id")?,
                email: r.try_get("email")?,
                password_hash: r.try_get("password_hash")?,
                first_name: r.try_get("first_name")?,
                last_name: r.try_get("last_name")?,
                is_active: r.try_get("is_active")?,
                created_at: r.try_get("created_at")?,
                updated_at: r.try_get("updated_at")?,
            };
            let role_name: Option<String> = r.try_get("role_name")?;
            Ok(Some((
                user,
                role_name.unwrap_or_else(|| "tenant_agent".to_string()),
            )))
        } else {
            Ok(None)
        }
    }

    pub async fn find_with_role_by_id(
        &self,
        id: Uuid,
        tenant_id: Option<Uuid>,
    ) -> Result<Option<(User, String)>, sqlx::Error> {
        let query_str = if tenant_id.is_some() {
            r#"SELECT u.id, u.tenant_id, u.role_id, u.email, u.password_hash, u.first_name, u.last_name, u.is_active, u.created_at, u.updated_at, r.name as role_name
               FROM users u LEFT JOIN roles r ON u.role_id = r.id WHERE u.id = $1 AND u.tenant_id = $2 AND u.deleted_at IS NULL"#
        } else {
            r#"SELECT u.id, u.tenant_id, u.role_id, u.email, u.password_hash, u.first_name, u.last_name, u.is_active, u.created_at, u.updated_at, r.name as role_name
               FROM users u LEFT JOIN roles r ON u.role_id = r.id WHERE u.id = $1 AND u.deleted_at IS NULL"#
        };

        let mut q = sqlx::query(query_str).bind(id);
        if let Some(t_id) = tenant_id {
            q = q.bind(t_id);
        }

        let row = q.fetch_optional(&*self.pool).await?;

        if let Some(r) = row {
            use sqlx::Row;
            let user = User {
                id: r.try_get("id")?,
                tenant_id: r.try_get("tenant_id")?,
                role_id: r.try_get("role_id")?,
                email: r.try_get("email")?,
                password_hash: r.try_get("password_hash")?,
                first_name: r.try_get("first_name")?,
                last_name: r.try_get("last_name")?,
                is_active: r.try_get("is_active")?,
                created_at: r.try_get("created_at")?,
                updated_at: r.try_get("updated_at")?,
            };
            let role_name: Option<String> = r.try_get("role_name")?;
            Ok(Some((
                user,
                role_name.unwrap_or_else(|| "tenant_agent".to_string()),
            )))
        } else {
            Ok(None)
        }
    }
}
