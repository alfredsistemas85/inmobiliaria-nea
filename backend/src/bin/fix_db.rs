use backend::{
    core::security::password::hash_password, infrastructure::database::users::UserRepository,
};
use sqlx::PgPool;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::from_path("../backend/.env").ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = Arc::new(PgPool::connect(&database_url).await?);

    let hash = hash_password("password123").unwrap();

    // 2. Update agent@test.com to have password and role
    sqlx::query("UPDATE users SET password_hash = $1, role = 'ADMIN_INMOBILIARIA' WHERE email = 'agent@test.com'")
        .bind(&hash)
        .execute(&*pool).await?;

    // 3. Create superadmin@inmobiliaria.com
    let superadmin_exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM users WHERE email = 'superadmin@inmobiliaria.com')",
    )
    .fetch_one(&*pool)
    .await?;

    if !superadmin_exists {
        sqlx::query(r#"
            INSERT INTO users (id, role, email, password_hash, first_name, last_name, is_active, email_verified_at)
            VALUES (gen_random_uuid(), 'SUPERADMIN', 'superadmin@inmobiliaria.com', $1, 'Super', 'Admin', true, CURRENT_TIMESTAMP)
        "#)
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
