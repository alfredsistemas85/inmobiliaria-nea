use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::models::whatsapp::{Conversation, ConversationListItem, Message};
use crate::models::common::PaginatedResponse;

#[derive(Clone)]
pub struct WhatsAppRepository {
    pool: Arc<PgPool>,
}

impl WhatsAppRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn find_or_create_conversation(
        &self,
        tenant_id: Uuid,
        client_id: Uuid,
    ) -> Result<Conversation, sqlx::Error> {
        // Try to find existing open conversation
        let existing = sqlx::query_as!(
            Conversation,
            r#"
            SELECT id, tenant_id, client_id, status, created_at, updated_at, 
                   last_message_at, unread_count, assigned_user_id, assigned_at, 
                   last_agent_response_at, deleted_at
            FROM conversations
            WHERE tenant_id = $1 AND client_id = $2 AND deleted_at IS NULL
            LIMIT 1
            "#,
            tenant_id,
            client_id
        )
        .fetch_optional(&*self.pool)
        .await?;

        if let Some(conv) = existing {
            return Ok(conv);
        }

        // Create new
        let new_conv = sqlx::query_as!(
            Conversation,
            r#"
            INSERT INTO conversations (tenant_id, client_id, status)
            VALUES ($1, $2, 'OPEN')
            RETURNING id, tenant_id, client_id, status, created_at, updated_at, 
                      last_message_at, unread_count, assigned_user_id, assigned_at, 
                      last_agent_response_at, deleted_at
            "#,
            tenant_id,
            client_id
        )
        .fetch_one(&*self.pool)
        .await?;

        Ok(new_conv)
    }

    pub async fn list_conversations(
        &self,
        tenant_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<PaginatedResponse<ConversationListItem>, sqlx::Error> {
        let total: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*)
            FROM conversations
            WHERE tenant_id = $1 AND deleted_at IS NULL
            "#
        )
        .bind(tenant_id)
        .fetch_one(&*self.pool)
        .await?;

        // Join to get client info and last message details
        let items = sqlx::query_as!(
            ConversationListItem,
            r#"
            SELECT 
                c.id, c.client_id, 
                cl.first_name as client_first_name, cl.last_name as client_last_name, cl.phone as client_phone,
                c.status, c.last_message_at, c.unread_count, c.assigned_user_id,
                c.assigned_at, c.last_agent_response_at,
                (SELECT content FROM messages WHERE conversation_id = c.id AND deleted_at IS NULL ORDER BY created_at DESC LIMIT 1) as last_message_content,
                (SELECT direction FROM messages WHERE conversation_id = c.id AND deleted_at IS NULL ORDER BY created_at DESC LIMIT 1) as last_message_direction
            FROM conversations c
            JOIN clients cl ON c.client_id = cl.id
            WHERE c.tenant_id = $1 AND c.deleted_at IS NULL
            ORDER BY c.last_message_at DESC NULLS LAST, c.created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            tenant_id,
            limit,
            offset
        )
        .fetch_all(&*self.pool)
        .await?;

        Ok(PaginatedResponse {
            data: items,
            total: total.0,
            limit,
            offset,
        })
    }

    pub async fn update_conversation_status(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        is_inbound: bool,
    ) -> Result<(), sqlx::Error> {
        if is_inbound {
            // Update unread count and last_message_at
            sqlx::query!(
                r#"
                UPDATE conversations
                SET last_message_at = CURRENT_TIMESTAMP, 
                    unread_count = COALESCE(unread_count, 0) + 1,
                    updated_at = CURRENT_TIMESTAMP
                WHERE id = $1 AND tenant_id = $2
                "#,
                conversation_id,
                tenant_id
            )
            .execute(&*self.pool)
            .await?;
        } else {
            // Outbound message: reset unread count
            sqlx::query!(
                r#"
                UPDATE conversations
                SET last_message_at = CURRENT_TIMESTAMP, 
                    unread_count = 0,
                    updated_at = CURRENT_TIMESTAMP
                WHERE id = $1 AND tenant_id = $2
                "#,
                conversation_id,
                tenant_id
            )
            .execute(&*self.pool)
            .await?;
        }
        Ok(())
    }

    pub async fn reset_unread_count(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE conversations
            SET unread_count = 0, updated_at = CURRENT_TIMESTAMP
            WHERE id = $1 AND tenant_id = $2
            "#,
            conversation_id,
            tenant_id
        )
        .execute(&*self.pool)
        .await?;
        Ok(())
    }

    pub async fn insert_message(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        external_id: Option<String>,
        direction: &str, // 'inbound' or 'outbound'
        sender_type: &str, // 'client', 'agent', 'bot'
        content: &str,
    ) -> Result<Option<Message>, sqlx::Error> {
        // If external_id is provided, check if it already exists
        if let Some(ref ext_id) = external_id {
            let exists: (i64,) = sqlx::query_as(
                "SELECT COUNT(*) FROM messages WHERE tenant_id = $1 AND external_id = $2"
            )
            .bind(tenant_id)
            .bind(ext_id)
            .fetch_one(&*self.pool)
            .await?;

            if exists.0 > 0 {
                // Already exists, skip insertion to prevent duplicates
                return Ok(None);
            }
        }

        let msg = sqlx::query_as!(
            Message,
            r#"
            INSERT INTO messages (tenant_id, conversation_id, external_id, direction, sender_type, content, status)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, tenant_id, conversation_id, sender_type, content, media_url, media_type, is_read, 
                      created_at, external_id, direction, status, is_ai_generated, deleted_at
            "#,
            tenant_id,
            conversation_id,
            external_id,
            direction,
            sender_type,
            content,
            if direction == "inbound" { "delivered" } else { "sent" } // default status
        )
        .fetch_one(&*self.pool)
        .await?;

        // Update conversation last_message_at and unread_count
        self.update_conversation_status(tenant_id, conversation_id, direction == "inbound").await?;

        Ok(Some(msg))
    }

    pub async fn list_messages(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<PaginatedResponse<Message>, sqlx::Error> {
        let total: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*)
            FROM messages
            WHERE tenant_id = $1 AND conversation_id = $2 AND deleted_at IS NULL
            "#
        )
        .bind(tenant_id)
        .bind(conversation_id)
        .fetch_one(&*self.pool)
        .await?;

        // Oldest to newest inside the window, but we want the most recent N messages if offset=0.
        // Wait, normally chat apps fetch the LATEST limit messages, ordered by time ascending in UI.
        // To do this, we order by created_at DESC, limit and offset, and then reverse in Rust or UI.
        // The user asked for "Orden: Más antiguos -> más nuevos. Preparado para conversaciones grandes."
        // If we do ORDER BY created_at ASC LIMIT 50 OFFSET 0, we get the FIRST 50 messages of the conversation.
        // Usually, chat gets the LAST 50 messages. But we'll follow standard ASC ordering for now and let UI handle scroll/fetching.
        // Wait, standard for GET /messages is ASC. If they want the last 50, they might need a custom query or UI fetches the last page.
        // Let's implement ORDER BY created_at ASC.
        let items = sqlx::query_as!(
            Message,
            r#"
            SELECT id, tenant_id, conversation_id, sender_type, content, media_url, media_type, is_read, 
                   created_at, external_id, direction, status, is_ai_generated, deleted_at
            FROM messages
            WHERE tenant_id = $1 AND conversation_id = $2 AND deleted_at IS NULL
            ORDER BY created_at ASC
            LIMIT $3 OFFSET $4
            "#,
            tenant_id,
            conversation_id,
            limit,
            offset
        )
        .fetch_all(&*self.pool)
        .await?;

        Ok(PaginatedResponse {
            data: items,
            total: total.0,
            limit,
            offset,
        })
    }

    pub async fn update_assignment(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        assigned_user_id: Option<Uuid>,
    ) -> Result<(), sqlx::Error> {
        if assigned_user_id.is_some() {
            sqlx::query!(
                "UPDATE conversations SET assigned_user_id = $1, assigned_at = CURRENT_TIMESTAMP WHERE id = $2 AND tenant_id = $3",
                assigned_user_id, conversation_id, tenant_id
            )
            .execute(&*self.pool)
            .await?;
        } else {
            sqlx::query!(
                "UPDATE conversations SET assigned_user_id = NULL, assigned_at = NULL WHERE id = $1 AND tenant_id = $2",
                conversation_id, tenant_id
            )
            .execute(&*self.pool)
            .await?;
        }
        Ok(())
    }

    pub async fn update_status(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        status: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE conversations SET status = $1 WHERE id = $2 AND tenant_id = $3",
            status, conversation_id, tenant_id
        )
        .execute(&*self.pool)
        .await?;
        Ok(())
    }
}
