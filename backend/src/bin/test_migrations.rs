use std::env;
use sqlx::{PgPool, Row};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&db_url).await?;

    println!("Checking _sqlx_migrations...");
    let result = sqlx::query("SELECT version FROM _sqlx_migrations ORDER BY version")
        .fetch_all(&pool)
        .await;

    match result {
        Ok(rows) => {
            println!("_sqlx_migrations found! Versions:");
            for row in rows {
                let version: i64 = row.try_get("version").unwrap_or(0);
                println!("- {}", version);
            }
        },
        Err(e) => println!("Error querying _sqlx_migrations: {:?}", e),
    }

    Ok(())
}
