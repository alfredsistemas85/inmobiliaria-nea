use std::env;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(sqlx::FromRow, Debug)]
pub struct Document {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub uploaded_by: Option<Uuid>,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub file_name: String,
    pub file_size: Option<i64>,
    pub mime_type: String,
    pub storage_path: Option<String>,
    pub version: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&db_url).await?;

    let result = sqlx::query_as::<_, Document>(
        "SELECT id, tenant_id, uploaded_by, entity_type, entity_id, file_name, file_size, mime_type, storage_path, version, created_at FROM documents LIMIT 1"
    )
    .fetch_optional(&pool)
    .await;

    match result {
        Ok(_) => println!("Query successful! The Document struct mapping is correct."),
        Err(e) => println!("DB Error: {:?}", e),
    }

    Ok(())
}
