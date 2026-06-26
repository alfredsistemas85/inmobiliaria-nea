use sqlx::postgres::PgPoolOptions;
use std::time::Instant;

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new().connect("postgresql://postgres.nwidmlslkkjyvpldzdry:xEnEizE41_2498@aws-1-us-west-2.pooler.supabase.com:5432/postgres").await.unwrap();

    let start = Instant::now();
    let rows: Vec<(i64,)> = sqlx::query_as("SELECT count(*) FROM properties")
        .fetch_all(&pool)
        .await
        .unwrap_or_default();
    println!("Properties count: {:?} in {:?}", rows, start.elapsed());

    let start = Instant::now();
    let rows: Vec<(i64,)> = sqlx::query_as("SELECT count(*) FROM clients")
        .fetch_all(&pool)
        .await
        .unwrap_or_default();
    println!("Clients count: {:?} in {:?}", rows, start.elapsed());

    let start = Instant::now();
    // Simulate complex dashboard query
    let _ = sqlx::query(r#"
        SELECT 
            (SELECT COUNT(*) FROM properties) as total_properties,
            (SELECT COUNT(*) FROM clients) as total_clients,
            (SELECT COUNT(*) FROM leads WHERE created_at >= date_trunc('month', CURRENT_DATE)) as new_leads,
            (SELECT COUNT(*) FROM appointments WHERE appointment_date >= CURRENT_DATE) as upcoming_appointments
    "#).fetch_one(&pool).await.unwrap();
    println!("Dashboard stats simulation in {:?}", start.elapsed());
}
