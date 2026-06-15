use backend::{
    core::security::password::hash_password,
    infrastructure::database::{roles::RoleRepository, users::UserRepository},
};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::from_path("../backend/.env").ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = Arc::new(PgPool::connect(&database_url).await?);

    // 1. Insert roles
    let queries = [
        "INSERT INTO roles (id, name, description) VALUES (gen_random_uuid(), 'super_admin', 'Dueño del SaaS') ON CONFLICT (name) DO NOTHING;",
        "INSERT INTO roles (id, name, description) VALUES (gen_random_uuid(), 'tenant_admin', 'Dueño de Inmobiliaria') ON CONFLICT (name) DO NOTHING;",
        "INSERT INTO roles (id, name, description) VALUES (gen_random_uuid(), 'tenant_agent', 'Agente Inmobiliario') ON CONFLICT (name) DO NOTHING;",
    ];

    for q in queries.iter() {
        sqlx::query(*q).execute(&*pool).await?;
    }

    let super_role_id: Uuid = sqlx::query_scalar("SELECT id FROM roles WHERE name = 'super_admin'").fetch_one(&*pool).await?;
    let admin_role_id: Uuid = sqlx::query_scalar("SELECT id FROM roles WHERE name = 'tenant_admin'").fetch_one(&*pool).await?;

    let hash = hash_password("password123").unwrap();

    // 2. Update agent@test.com to have password and role
    sqlx::query("UPDATE users SET password_hash = $1, role_id = $2 WHERE email = 'agent@test.com'")
        .bind(&hash)
        .bind(admin_role_id)
        .execute(&*pool).await?;

    // 3. Create superadmin@inmobiliaria.com
    let superadmin_exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE email = 'superadmin@inmobiliaria.com')").fetch_one(&*pool).await?;
    
    if !superadmin_exists {
        sqlx::query(r#"
            INSERT INTO users (id, role_id, email, password_hash, first_name, last_name, is_active, email_verified_at)
            VALUES (gen_random_uuid(), $1, 'superadmin@inmobiliaria.com', $2, 'Super', 'Admin', true, CURRENT_TIMESTAMP)
        "#)
        .bind(super_role_id)
        .bind(&hash)
        .execute(&*pool).await?;
    }

    // 4. Update tenants status to ACTIVE
    sqlx::query("UPDATE tenants SET status = 'ACTIVE' WHERE status IN ('APPROVED', 'PENDING')")
        .execute(&*pool)
        .await?;

    println!("Users and tenants setup successfully!");

    Ok(())
}
