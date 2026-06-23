use std::env;
use sqlx::{PgPool, Connection, migrate::Migrator};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&db_url).await?;

    let migrator = Migrator::new(Path::new("./migrations")).await?;
    
    // We can manually insert the applied migrations into _sqlx_migrations
    for migration in migrator.iter() {
        let version = migration.version;
        let description = &migration.description;
        let checksum = &migration.checksum;
        
        let exists: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM _sqlx_migrations WHERE version = $1")
            .bind(version)
            .fetch_one(&pool)
            .await?;
            
        if exists.0 == 0 {
            // Check if this migration corresponds to a table that already exists
            // For example, 20240101000000_initial_schema creates `plans` table.
            let table_exists: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM information_schema.tables WHERE table_name = 'plans'")
                .fetch_one(&pool)
                .await?;
                
            if table_exists.0 > 0 && version <= 20240625000000 {
                // The database already has the old tables, so we just mark this old migration as applied!
                println!("Marking old migration {} as applied...", version);
                sqlx::query("INSERT INTO _sqlx_migrations (version, description, success, checksum, execution_time) VALUES ($1, $2, true, $3, 0)")
                    .bind(version)
                    .bind(description)
                    .bind(&**checksum)
                    .execute(&pool)
                    .await?;
            }
        }
    }
    
    println!("Done marking baseline migrations!");
    Ok(())
}
