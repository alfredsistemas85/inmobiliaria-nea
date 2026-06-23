use std::env;
use sqlx::{PgPool, migrate::Migrator};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&db_url).await?;

    let migrator = Migrator::new(Path::new("./migrations")).await?;
    
    for migration in migrator.iter() {
        if migration.version == 20240620000000 {
            let checksum = &migration.checksum;
            println!("Updating checksum for 20240620000000...");
            sqlx::query("UPDATE _sqlx_migrations SET checksum = $1 WHERE version = 20240620000000")
                .bind(&**checksum)
                .execute(&pool)
                .await?;
            println!("Done.");
        }
    }
    
    Ok(())
}
