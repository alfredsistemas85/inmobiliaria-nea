use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new().connect("postgresql://postgres.nwidmlslkkjyvpldzdry:xEnEizE41_2498@aws-1-us-west-2.pooler.supabase.com:5432/postgres").await.unwrap();
    
    // Check missing indexes (FKs without indexes)
    let rows: Vec<(String, String)> = sqlx::query_as(
        r#"
        SELECT
            tc.table_name::text,
            kcu.column_name::text
        FROM
            information_schema.table_constraints tc
            JOIN information_schema.key_column_usage kcu
              ON tc.constraint_name = kcu.constraint_name
              AND tc.table_schema = kcu.table_schema
        WHERE tc.constraint_type = 'FOREIGN KEY'
          AND NOT EXISTS (
              SELECT 1 FROM pg_indexes 
              WHERE tablename = tc.table_name 
                AND indexdef LIKE '%' || kcu.column_name || '%'
          )
        "#
    ).fetch_all(&pool).await.unwrap_or_default();
    
    println!("--- FKs without indexes ---");
    for (table, column) in rows {
        println!("{}.{}", table, column);
    }
}
