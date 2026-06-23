-- Módulo SuperAdmin: Estado de Inmobiliarias, Monitoreo y Soporte

-- 1. Agregar estado a tenants (PENDING, ACTIVE, SUSPENDED, DELETED)
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS status VARCHAR(50) DEFAULT 'ACTIVE';

-- Actualizar el estado inicial basado en is_active actual
UPDATE tenants SET status = 'ACTIVE' WHERE is_active = true;
UPDATE tenants SET status = 'SUSPENDED' WHERE is_active = false;

-- 2. Monitoreo de Errores Globales (system_errors)
CREATE TABLE system_errors (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID REFERENCES tenants(id) ON DELETE SET NULL, -- Puede ser null si es error global
    error_type VARCHAR(100) NOT NULL, -- e.g., '500_INTERNAL_ERROR', 'WEBHOOK_FAILURE', 'DB_INCONSISTENCY'
    endpoint VARCHAR(255),
    method VARCHAR(10),
    error_message TEXT NOT NULL,
    stack_trace TEXT,
    payload JSONB,
    resolved BOOLEAN DEFAULT false,
    resolved_at TIMESTAMP WITH TIME ZONE,
    resolved_by UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_system_errors_tenant ON system_errors(tenant_id);
CREATE INDEX idx_system_errors_type ON system_errors(error_type);
CREATE INDEX idx_system_errors_created_at ON system_errors(created_at);

-- 3. Soporte Técnico (Tickets)
CREATE TABLE support_tickets (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE SET NULL, -- El usuario de la inmobiliaria que creó el ticket
    subject VARCHAR(255) NOT NULL,
    status VARCHAR(50) DEFAULT 'OPEN', -- OPEN, IN_PROGRESS, RESOLVED, CLOSED
    priority VARCHAR(50) DEFAULT 'NORMAL', -- LOW, NORMAL, HIGH, URGENT
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_support_tickets_tenant ON support_tickets(tenant_id);
CREATE INDEX idx_support_tickets_status ON support_tickets(status);

CREATE TABLE support_ticket_messages (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    ticket_id UUID NOT NULL REFERENCES support_tickets(id) ON DELETE CASCADE,
    sender_id UUID REFERENCES users(id) ON DELETE SET NULL, -- Puede ser un tenant_user o un super_admin
    is_superadmin BOOLEAN DEFAULT false, -- Para distinguir visualmente fácilmente
    message TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_support_ticket_messages_ticket ON support_ticket_messages(ticket_id);
