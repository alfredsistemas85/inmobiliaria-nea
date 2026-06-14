-- 1. Extend Conversations table (assigned_user_id and status already exist)
ALTER TABLE conversations ADD COLUMN assigned_at TIMESTAMP WITH TIME ZONE;
ALTER TABLE conversations ADD COLUMN last_agent_response_at TIMESTAMP WITH TIME ZONE;

-- Add index for status and assignments
CREATE INDEX IF NOT EXISTS idx_conversations_tenant_status ON conversations(tenant_id, status);
CREATE INDEX IF NOT EXISTS idx_conversations_assigned_user ON conversations(assigned_user_id);

-- 2. Notifications table
CREATE TABLE IF NOT EXISTS notifications (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE CASCADE, -- NULL means broadcast to tenant
    type VARCHAR(50) NOT NULL, -- 'NEW_LEAD', 'NEW_MESSAGE', 'ASSIGNED', 'APPOINTMENT_CREATED', etc.
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    read_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_notifications_tenant_user_unread ON notifications(tenant_id, user_id, read_at) WHERE read_at IS NULL;

-- 3. Scheduled Tasks for Persistent Scheduler
CREATE TABLE IF NOT EXISTS scheduled_tasks (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    task_type VARCHAR(50) NOT NULL, -- 'APPOINTMENT_REMINDER', 'LEAD_FOLLOWUP', 'CONVERSATION_FOLLOWUP'
    payload JSONB NOT NULL,
    run_at TIMESTAMP WITH TIME ZONE NOT NULL,
    status VARCHAR(50) DEFAULT 'PENDING' NOT NULL, -- 'PENDING', 'RUNNING', 'COMPLETED', 'FAILED'
    locked_at TIMESTAMP WITH TIME ZONE,
    executed_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Index for the worker process to efficiently grab pending tasks
CREATE INDEX IF NOT EXISTS idx_scheduled_tasks_pending_run_at ON scheduled_tasks(status, run_at) WHERE status = 'PENDING';
