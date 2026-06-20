-- Add deleted_at to users table
ALTER TABLE users ADD COLUMN deleted_at TIMESTAMP WITH TIME ZONE;
CREATE INDEX idx_users_deleted_at ON users(deleted_at);

-- Create performance and security indices
CREATE INDEX idx_leads_tenant_status ON leads(tenant_id, status);
CREATE INDEX idx_appointments_tenant_scheduled ON appointments(tenant_id, scheduled_at);
-- CREATE INDEX idx_conversations_tenant_last_message ON conversations(tenant_id, last_message_at); -- Note: idx_conversations_last_message already exists from phase 3
CREATE INDEX idx_clients_tenant_name ON clients(tenant_id, first_name, last_name);
CREATE INDEX idx_properties_tenant_status ON properties(tenant_id, status);
