use reqwest::{Client, StatusCode};
use serde_json::json;
use std::env;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Clone)]
pub struct EvolutionClient {
    client: Client,
    api_url: String,
    api_key: String,
    instance: String,
}

impl EvolutionClient {
    pub fn new() -> Self {
        let api_url =
            env::var("EVOLUTION_API_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
        let api_key = env::var("EVOLUTION_API_KEY").unwrap_or_else(|_| "".to_string());
        let instance = env::var("EVOLUTION_INSTANCE").unwrap_or_else(|_| "default".to_string());

        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to build HTTP client for Evolution");

        Self {
            client,
            api_url,
            api_key,
            instance,
        }
    }

    pub async fn send_message(&self, phone: &str, message: &str) -> Result<(), String> {
        let url = format!("{}/message/sendText/{}", self.api_url, self.instance);

        let payload = json!({
            "number": phone,
            "text": message
        });

        let mut retries = 3;
        let mut delay = Duration::from_secs(1);

        loop {
            let req = self
                .client
                .post(&url)
                .header("apikey", &self.api_key)
                .json(&payload)
                .send()
                .await;

            match req {
                Ok(resp) => {
                    if resp.status().is_success() {
                        return Ok(());
                    } else {
                        let status = resp.status();
                        let text = resp.text().await.unwrap_or_default();

                        // If it's a 4xx error (except 429), it's probably a bad request, don't retry.
                        if status.is_client_error() && status != StatusCode::TOO_MANY_REQUESTS {
                            return Err(format!(
                                "Client error sending WhatsApp: {} - {}",
                                status, text
                            ));
                        }

                        tracing::warn!(
                            "Failed to send WhatsApp message. Status: {}. Response: {}",
                            status,
                            text
                        );
                    }
                }
                Err(e) => {
                    tracing::warn!("Network error sending WhatsApp message: {}", e);
                }
            }

            retries -= 1;
            if retries == 0 {
                return Err("Max retries reached for sending WhatsApp message".to_string());
            }

            sleep(delay).await;
            delay *= 2; // Exponential backoff
        }
    }

    pub async fn send_media(&self, phone: &str, message: &str, media_url: &str, file_name: &str) -> Result<(), String> {
        let url = format!("{}/message/sendMedia/{}", self.api_url, self.instance);

        let payload = json!({
            "number": phone,
            "mediatype": "document",
            "media": media_url,
            "fileName": file_name,
            "caption": message
        });

        let mut retries = 3;
        let mut delay = Duration::from_secs(1);

        loop {
            let req = self
                .client
                .post(&url)
                .header("apikey", &self.api_key)
                .json(&payload)
                .send()
                .await;

            match req {
                Ok(resp) => {
                    if resp.status().is_success() {
                        return Ok(());
                    } else {
                        let status = resp.status();
                        let text = resp.text().await.unwrap_or_default();
                        
                        if status.is_client_error() && status != StatusCode::TOO_MANY_REQUESTS {
                            return Err(format!(
                                "Client error sending WhatsApp media: {} - {}",
                                status, text
                            ));
                        }
                        tracing::warn!("Failed to send WhatsApp media. Status: {}. Response: {}", status, text);
                    }
                }
                Err(e) => tracing::warn!("Network error sending WhatsApp media: {}", e),
            }

            retries -= 1;
            if retries == 0 {
                return Err("Max retries reached for sending WhatsApp media".to_string());
            }

            sleep(delay).await;
            delay *= 2;
        }
    }

    pub async fn create_instance(&self, instance_name: &str) -> Result<serde_json::Value, String> {
        let url = format!("{}/instance/create", self.api_url);
        let payload = json!({
            "instanceName": instance_name,
            "token": self.api_key,
            "qrcode": true
        });

        let resp = self
            .client
            .post(&url)
            .header("apikey", &self.api_key)
            .json(&payload)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if resp.status().is_success() {
            resp.json::<serde_json::Value>()
                .await
                .map_err(|e| e.to_string())
        } else {
            Err(format!("Error creating instance: {}", resp.status()))
        }
    }

    pub async fn get_instance_state(
        &self,
        instance_name: &str,
    ) -> Result<serde_json::Value, String> {
        let url = format!(
            "{}/instance/connectionState/{}",
            self.api_url, instance_name
        );

        let resp = self
            .client
            .get(&url)
            .header("apikey", &self.api_key)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if resp.status().is_success() {
            resp.json::<serde_json::Value>()
                .await
                .map_err(|e| e.to_string())
        } else {
            Err(format!("Error getting state: {}", resp.status()))
        }
    }

    pub async fn connect_instance(&self, instance_name: &str) -> Result<serde_json::Value, String> {
        let url = format!("{}/instance/connect/{}", self.api_url, instance_name);

        let resp = self
            .client
            .get(&url)
            .header("apikey", &self.api_key)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if resp.status().is_success() {
            resp.json::<serde_json::Value>()
                .await
                .map_err(|e| e.to_string())
        } else {
            Err(format!("Error connecting: {}", resp.status()))
        }
    }

    pub async fn logout_instance(&self, instance_name: &str) -> Result<(), String> {
        let url = format!("{}/instance/logout/{}", self.api_url, instance_name);

        let resp = self
            .client
            .delete(&url)
            .header("apikey", &self.api_key)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if resp.status().is_success() {
            Ok(())
        } else {
            Err(format!("Error logout: {}", resp.status()))
        }
    }
}
