use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SendMessageDto {
    pub phone: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct WebhookPayload {
    pub event: Option<String>,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct AssignConversationDto {
    pub user_id: uuid::Uuid,
}
