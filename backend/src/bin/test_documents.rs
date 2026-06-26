use sqlx::PgPool;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&db_url).await?;

    let result = sqlx::query(
        "SELECT id, tenant_id, uploaded_by, entity_type, entity_id, file_name, file_size, mime_type, storage_path, version, created_at FROM documents LIMIT 1"
    )
    .fetch_optional(&pool)
    .await;

    match result {
        Ok(_) => println!("Query successful! The documents table and columns exist."),
        Err(e) => println!("DB Error: {:?}", e),
    }

    Ok(())
}
