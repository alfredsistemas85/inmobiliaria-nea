use sqlx::PgPool;
use std::env;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&db_url).await?;

    let tenant_id = Uuid::parse_str("82547acc-c6f8-4ee3-ac79-e4ba68b181ac")?;
    let contract_id = Uuid::parse_str("004d9cfa-cf15-4e06-9c35-7d828e87f30b")?;
    
    let mut tx = pool.begin().await?;
    
    let id = sqlx::query_scalar::<_, Uuid>(
        r#"
        INSERT INTO contract_snapshots (tenant_id, contract_id, snapshot_json)
        VALUES ($1, $2, $3)
        RETURNING id
        "#
    )
    .bind(tenant_id)
    .bind(contract_id)
    .bind(serde_json::json!({}))
    .fetch_one(&mut *tx)
    .await?;

    println!("Inserted snapshot: {}", id);
    
    tx.commit().await?;

    Ok(())
}
