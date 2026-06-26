use std::env;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    
    // Forzar el url con sslmode=require en caso de que no lo tenga
    let mut db_url = env::var("DATABASE_URL").expect("DATABASE_URL no encontrada en .env");
    
    println!("=== TEST DE CONEXION RUST SQLX ===");
    println!("1. Leyendo DATABASE_URL desde .env...");
    println!("Host parseado: {}", db_url.replace("xEnEizE41_2498", "****"));
    
    if !db_url.contains("sslmode") {
        db_url.push_str("?sslmode=require");
        println!("2. Inyectando sslmode=require -> {}", db_url.replace("xEnEizE41_2498", "****"));
    }

    println!("3. Intentando conexión a PostgreSQL (Timeout 10s)...");
    
    let pool_result = PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(std::time::Duration::from_secs(10))
        .connect(&db_url)
        .await;

    match pool_result {
        Ok(pool) => {
            println!("✅ TCP OK. SSL OK. Autenticación OK.");
            println!("4. Ejecutando SELECT 1...");
            match sqlx::query("SELECT 1 as num").fetch_one(&pool).await {
                Ok(row) => {
                    use sqlx::Row;
                    println!("✅ SELECT exitoso. Resultado: {:?}", row.get::<i32, _>("num"));
                }
                Err(e) => {
                    println!("❌ Error en SELECT: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ ERROR FATAL DE CONEXION SQLX:");
            println!("{:?}", e);
        }
    }
    
    Ok(())
}
