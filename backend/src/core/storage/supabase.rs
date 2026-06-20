use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;
use std::env;
use super::StorageProvider;

#[derive(Clone)]
pub struct SupabaseStorageProvider {
    client: Client,
    url: String,
    service_key: String,
}

impl SupabaseStorageProvider {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            url: env::var("SUPABASE_URL").unwrap_or_default(),
            service_key: env::var("SUPABASE_SERVICE_ROLE_KEY").unwrap_or_default(),
        }
    }
}

#[async_trait]
impl StorageProvider for SupabaseStorageProvider {
    async fn upload_document(
        &self,
        bucket: &str,
        path: &str,
        content: Vec<u8>,
        content_type: &str,
    ) -> Result<String, String> {
        let endpoint = format!("{}/storage/v1/object/{}/{}", self.url, bucket, path);
        
        let res = self.client.post(&endpoint)
            .header("apikey", &self.service_key)
            .header("Authorization", format!("Bearer {}", self.service_key))
            .header("Content-Type", content_type)
            .body(content)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            return Err(format!("Upload failed: {} - {}", status, text));
        }

        Ok(path.to_string())
    }

    async fn generate_signed_url(
        &self,
        bucket: &str,
        path: &str,
        expires_in_seconds: u32,
    ) -> Result<String, String> {
        let endpoint = format!("{}/storage/v1/object/sign/{}/{}", self.url, bucket, path);
        
        let payload = json!({
            "expiresIn": expires_in_seconds
        });

        let res = self.client.post(&endpoint)
            .header("apikey", &self.service_key)
            .header("Authorization", format!("Bearer {}", self.service_key))
            .json(&payload)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            return Err(format!("Sign failed: {} - {}", status, text));
        }

        #[derive(serde::Deserialize)]
        struct SignResponse {
            #[serde(rename = "signedURL")]
            signed_url: String,
        }

        let data: SignResponse = res.json().await.map_err(|e| e.to_string())?;
        
        Ok(format!("{}{}", self.url, data.signed_url))
    }
}
