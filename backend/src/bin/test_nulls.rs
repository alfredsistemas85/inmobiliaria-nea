use sqlx::{PgPool, Row};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&db_url).await?;

    let count: i64 = sqlx::query("SELECT COUNT(*) FROM documents")
        .fetch_one(&pool)
        .await?
        .try_get(0)?;
    println!("Total documents: {}", count);

    let null_uploaded_by: i64 =
        sqlx::query("SELECT COUNT(*) FROM documents WHERE uploaded_by IS NULL")
            .fetch_one(&pool)
            .await?
            .try_get(0)?;
    println!("Documents with NULL uploaded_by: {}", null_uploaded_by);

    let null_file_size: i64 = sqlx::query("SELECT COUNT(*) FROM documents WHERE file_size IS NULL")
        .fetch_one(&pool)
        .await?
        .try_get(0)?;
    println!("Documents with NULL file_size: {}", null_file_size);

    Ok(())
}
