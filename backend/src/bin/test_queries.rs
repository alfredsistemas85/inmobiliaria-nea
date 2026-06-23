
use sqlx::PgPool;
use std::env;

use uuid::Uuid;
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&db_url).await?;
    
    println!("Connected to database. Testing WhatsApp query...");
    let tenant_id = Uuid::nil(); // arbitrary for syntax check
    let res = sqlx::query!(
        "SELECT id, tenant_id, instance_name, status, qr_code, phone_connected FROM whatsapp_instances WHERE tenant_id = $1 LIMIT 1",
        tenant_id
    )
    .fetch_optional(&pool)
    .await;
    
    match res {
        Ok(_) => println!("WhatsApp query succeeded."),
        Err(e) => println!("WhatsApp query failed: {:?}", e),
    }

    println!("Testing Liquidations query...");
    let res2 = sqlx::query!(
        r#"
        SELECT 
            i.amount, 
            i.commission, 
            p.title as property_title, 
            ''::TEXT as owner_name 
        FROM invoices i
        JOIN contracts ct ON i.contract_id = ct.id
        JOIN properties p ON ct.property_id = p.id
        WHERE i.tenant_id = $1 AND i.status = 'PAID'
        "#,
        tenant_id
    )
    .fetch_all(&pool)
    .await;

    match res2 {
        Ok(_) => println!("Liquidations query succeeded."),
        Err(e) => println!("Liquidations query failed: {:?}", e),
    }

    Ok(())
}
