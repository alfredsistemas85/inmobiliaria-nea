use sqlx::PgPool;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&db_url).await?;

    let row: (Vec<u8>,) =
        sqlx::query_as("SELECT checksum FROM _sqlx_migrations WHERE version = 20240616000000")
            .fetch_one(&pool)
            .await?;

    println!("Checksum in DB: {:?}", row.0);
    Ok(())
}
