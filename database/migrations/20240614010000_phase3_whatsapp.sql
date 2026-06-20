-- Añadir campos faltantes a conversations
ALTER TABLE conversations ADD COLUMN last_message_at TIMESTAMP WITH TIME ZONE;
ALTER TABLE conversations ADD COLUMN unread_count INT DEFAULT 0;
ALTER TABLE conversations ADD COLUMN assigned_user_id UUID REFERENCES users(id) ON DELETE SET NULL;
ALTER TABLE conversations ADD COLUMN deleted_at TIMESTAMP WITH TIME ZONE;

-- Añadir campos faltantes a messages
ALTER TABLE messages ADD COLUMN external_id VARCHAR(255);
ALTER TABLE messages ADD COLUMN direction VARCHAR(50) DEFAULT 'inbound'; -- 'inbound', 'outbound'
ALTER TABLE messages ADD COLUMN status VARCHAR(50) DEFAULT 'pending'; -- 'pending', 'sent', 'delivered', 'read', 'failed'
ALTER TABLE messages ADD COLUMN is_ai_generated BOOLEAN DEFAULT FALSE;
ALTER TABLE messages ADD COLUMN deleted_at TIMESTAMP WITH TIME ZONE;

-- Modificar tabla messages si sender_type ya existe, podríamos mantenerlo o basarnos en direction.
-- En la migración inicial sender_type es VARCHAR(50). Mantenemos todo junto.

-- Añadir restricciones y constraints para external_id
-- external_id debe ser UNIQUE por tenant para evitar duplicación de webhooks.
CREATE UNIQUE INDEX idx_messages_external_id_tenant ON messages(tenant_id, external_id) WHERE external_id IS NOT NULL;

-- Índices adicionales para rendimiento
CREATE INDEX idx_conversations_last_message ON conversations(tenant_id, last_message_at DESC);
CREATE INDEX idx_messages_deleted_at ON messages(deleted_at);
CREATE INDEX idx_conversations_deleted_at ON conversations(deleted_at);
