use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug)]
struct DocumentRecord {
    id: Uuid,
    entity_type: String,
    entity_id: Uuid,
    file_name: String,
    storage_path: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    println!("=== AUDIT START ===");
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL missing");
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&db_url)
        .await?;

    let property_id_str = "bf952d76-2495-40aa-87ea-6bab56f0f77a";
    let property_id = Uuid::parse_str(property_id_str).unwrap();

    println!("Querying documents for property_id = {}", property_id);

    let docs = sqlx::query_as::<_, DocumentRecord>(
        "SELECT id, entity_type, entity_id, file_name, storage_path FROM documents WHERE entity_id = $1"
    )
    .bind(property_id)
    .fetch_all(&pool)
    .await?;

    println!("Found {} documents:", docs.len());
    for doc in &docs {
        println!("{:?}", doc);
    }

    println!("\nTesting Supabase Signed URL generation...");
    let supabase_url = env::var("SUPABASE_URL").unwrap_or_default();
    let service_role_key = env::var("SUPABASE_SERVICE_ROLE_KEY").unwrap_or_default();
    let bucket_name =
        env::var("SUPABASE_DOCUMENTS_BUCKET").unwrap_or_else(|_| "certificados".to_string());

    println!("Supabase URL: {}", supabase_url);
    println!("Bucket: {}", bucket_name);

    if docs.is_empty() {
        println!("No docs to test signed URL.");
        return Ok(());
    }

    let client = reqwest::Client::new();
    let path = docs[0].storage_path.as_deref().unwrap_or_default();
    let url = format!(
        "{}/storage/v1/object/sign/{}/{}",
        supabase_url, bucket_name, path
    );
    println!("Requesting Signed URL from: {}", url);

    let res = client
        .post(&url)
        .bearer_auth(&service_role_key)
        .header("apikey", &service_role_key)
        .json(&serde_json::json!({ "expiresIn": 3600 }))
        .send()
        .await?;

    println!("Supabase Status: {}", res.status());
    let body = res.text().await?;
    println!("Supabase Response: {}", body);

    println!("=== AUDIT END ===");
    Ok(())
}
