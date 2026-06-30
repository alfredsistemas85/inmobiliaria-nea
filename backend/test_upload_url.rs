use std::env;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let supabase_url = env::var("SUPABASE_URL").unwrap();
    let service_role_key = env::var("SUPABASE_SERVICE_ROLE_KEY").unwrap();
    let bucket_name = "certificados";
    let path = "12345678-1234-1234-1234-123456789012/12345678-1234-1234-1234-123456789012/original.pdf";

    let url = format!(
        "{}/storage/v1/object/upload/sign/{}/{}",
        supabase_url, bucket_name, path
    );

    let client = Client::new();
    let res = client
        .post(&url)
        .bearer_auth(&service_role_key)
        .header("apikey", &service_role_key)
        .send()
        .await?;

    println!("Status: {}", res.status());
    println!("Body: {}", res.text().await?);

    Ok(())
}
