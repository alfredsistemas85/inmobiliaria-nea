use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize)]
struct SignRequest {
    #[serde(rename = "expiresIn")]
    expires_in: i32,
}

#[derive(Deserialize)]
struct SignResponse {
    #[serde(rename = "signedURL")]
    signed_url: Option<String>,
    #[serde(rename = "url")]
    url: Option<String>,
}

pub struct SupabaseStorage {
    client: Client,
    supabase_url: String,
    service_role_key: String,
    bucket_name: String,
}

impl SupabaseStorage {
    pub fn new() -> Self {
        let supabase_url = env::var("SUPABASE_URL").unwrap_or_default();
        let service_role_key = env::var("SUPABASE_SERVICE_ROLE_KEY").unwrap_or_default();
        // default bucket changed to "certificados" based on Supabase setup
        let bucket_name =
            env::var("SUPABASE_DOCUMENTS_BUCKET").unwrap_or_else(|_| "certificados".to_string());

        Self {
            client: Client::new(),
            supabase_url,
            service_role_key,
            bucket_name,
        }
    }

    // Creates a signed URL for uploading a file directly from the client
    pub async fn create_upload_url(&self, path: &str) -> Result<String, String> {
        let url = format!(
            "{}/storage/v1/object/upload/sign/{}/{}",
            self.supabase_url, self.bucket_name, path
        );

        let res = self
            .client
            .post(&url)
            .bearer_auth(&self.service_role_key)
            .header("apikey", &self.service_role_key)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !res.status().is_success() {
            let status = res.status();
            let body = res.text().await.unwrap_or_default();
            return Err(format!("Failed to create upload URL: {} - {}", status, body));
        }

        let data: SignResponse = res.json().await.map_err(|e| e.to_string())?;

        // Supabase typically returns a relative path like "/storage/v1/object/upload/sign/..."
        // or a full URL in 'url'
        if let Some(mut signed) = data.url.or(data.signed_url) {
            if signed.starts_with("/") {
                // Supabase API returns relative paths without "/storage/v1"
                signed = format!("{}/storage/v1{}", self.supabase_url, signed);
            }
            Ok(signed)
        } else {
            Err("No signed URL in response".to_string())
        }
    }

    // Creates a signed URL to download or view a file
    pub async fn create_download_url(&self, path: &str, expires_in: i32) -> Result<String, String> {
        let url = format!(
            "{}/storage/v1/object/sign/{}/{}",
            self.supabase_url, self.bucket_name, path
        );

        let res = self
            .client
            .post(&url)
            .bearer_auth(&self.service_role_key)
            .header("apikey", &self.service_role_key)
            .json(&SignRequest { expires_in })
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !res.status().is_success() {
            return Err(format!("Failed to create download URL: {}", res.status()));
        }

        let data: SignResponse = res.json().await.map_err(|e| e.to_string())?;

        if let Some(mut signed) = data.signed_url {
            if signed.starts_with("/") {
                signed = format!("{}/storage/v1{}", self.supabase_url, signed);
            }
            Ok(signed)
        } else {
            Err("No signed URL in response".to_string())
        }
    }
}
