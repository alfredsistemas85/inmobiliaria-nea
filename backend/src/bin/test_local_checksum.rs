use sqlx::migrate::Migrator;
use std::env;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let migrator = Migrator::new(Path::new("./migrations")).await?;

    for migration in migrator.iter() {
        if migration.version == 20240616000000 {
            println!("Local Checksum: {:?}", &*migration.checksum);
        }
    }
    Ok(())
}
