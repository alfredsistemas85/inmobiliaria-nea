use std::env;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&db_url).await?;

    println!("Running sqlx::migrate! on live DB...");
    let result = sqlx::migrate!("./migrations")
        .run(&pool)
        .await;

    match result {
        Ok(_) => println!("Migrations completed successfully on live DB!"),
        Err(e) => println!("Migration panic/error: {:?}", e),
    }

    Ok(())
}
