-- Update appointments table
ALTER TABLE appointments 
ADD COLUMN confirmed_at TIMESTAMP WITH TIME ZONE,
ADD COLUMN cancelled_at TIMESTAMP WITH TIME ZONE,
ADD COLUMN confirmation_sent_at TIMESTAMP WITH TIME ZONE;

-- We don't alter the VARCHAR(50) status column type, but we will use the new values in application logic:
-- 'PENDING_CONFIRMATION', 'CONFIRMED', 'CANCELLED', 'COMPLETED', 'NO_SHOW', 'RESCHEDULED'

-- Create appointment_notifications
CREATE TABLE appointment_notifications (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    appointment_id UUID NOT NULL REFERENCES appointments(id) ON DELETE CASCADE,
    notification_type VARCHAR(50) NOT NULL, -- 'REMINDER_24H', 'REMINDER_2H', 'CONFIRMATION'
    sent_at TIMESTAMP WITH TIME ZONE,
    status VARCHAR(50) DEFAULT 'PENDING', -- 'PENDING', 'SENT', 'FAILED'
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create appointment_audit_logs
CREATE TABLE appointment_audit_logs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    appointment_id UUID NOT NULL REFERENCES appointments(id) ON DELETE CASCADE,
    action VARCHAR(255) NOT NULL, -- e.g., 'STATUS_CHANGED', 'REMINDER_SENT'
    old_status VARCHAR(50),
    new_status VARCHAR(50),
    performed_by VARCHAR(255), -- could be 'SYSTEM', 'CLIENT', or user_id
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Add indexes for new tables
CREATE INDEX idx_appointment_notifications_tenant ON appointment_notifications(tenant_id);
CREATE INDEX idx_appointment_notifications_appointment ON appointment_notifications(appointment_id);
CREATE INDEX idx_appointment_audit_logs_appointment ON appointment_audit_logs(appointment_id);
