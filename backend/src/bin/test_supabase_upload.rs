use reqwest::Client;
use std::env;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    
    let supabase_url = env::var("SUPABASE_URL").unwrap_or_default();
    let service_role_key = env::var("SUPABASE_SERVICE_ROLE_KEY").unwrap_or_default();
    let bucket_name = "documents";
    let path = "test-tenant/property/123/image.jpg";
    
    let url = format!("{}/storage/v1/object/upload/sign/{}/{}", supabase_url, bucket_name, path);
    println!("Testing URL: {}", url);
    
    let client = Client::new();
    let res = client.post(&url)
        .bearer_auth(&service_role_key)
        .header("apikey", &service_role_key)
        .send()
        .await;
        
    match res {
        Ok(response) => {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            println!("Status: {}", status);
            println!("Body: {}", body);
        },
        Err(e) => {
            println!("Request Failed: {}", e);
        }
    }
}
