use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Conversation {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub client_id: Uuid,
    pub status: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub last_message_at: Option<DateTime<Utc>>,
    pub unread_count: Option<i32>,
    pub assigned_user_id: Option<Uuid>,
    pub assigned_at: Option<DateTime<Utc>>,
    pub last_agent_response_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub conversation_id: Uuid,
    pub sender_type: String,
    pub content: Option<String>,
    pub media_url: Option<String>,
    pub media_type: Option<String>,
    pub is_read: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub external_id: Option<String>,
    pub direction: Option<String>,
    pub status: Option<String>,
    pub is_ai_generated: Option<bool>,
    pub deleted_at: Option<DateTime<Utc>>,
}

// Joined struct for Conversation List endpoint
#[derive(Debug, Serialize, Deserialize)]
pub struct ConversationListItem {
    pub id: Uuid,
    pub client_id: Uuid,
    pub client_first_name: Option<String>,
    pub client_last_name: Option<String>,
    pub client_phone: String,
    pub status: Option<String>,
    pub last_message_at: Option<DateTime<Utc>>,
    pub unread_count: Option<i32>,
    pub assigned_user_id: Option<Uuid>,
    pub assigned_at: Option<DateTime<Utc>>,
    pub last_agent_response_at: Option<DateTime<Utc>>,
    pub last_message_content: Option<String>,
    pub last_message_direction: Option<String>,
}
