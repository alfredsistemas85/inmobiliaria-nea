-- Migration: 20240621000000_fase4_productivity_modules.sql
-- Description: Implement tables for Calendar Integration, Documents, and Digital Signatures

-- 1. Calendar Integrations
CREATE TABLE IF NOT EXISTS calendar_integrations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    user_id UUID NOT NULL REFERENCES users(id),
    provider VARCHAR(20) NOT NULL, -- 'google', 'outlook'
    external_email VARCHAR(255),
    access_token TEXT NOT NULL,
    refresh_token TEXT,
    token_expires_at TIMESTAMPTZ,
    calendar_id VARCHAR(255),
    active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, user_id, provider)
);

-- 2. Calendar Sync Events
CREATE TABLE IF NOT EXISTS calendar_sync_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    user_id UUID NOT NULL REFERENCES users(id),
    crm_event_id UUID NOT NULL, -- Polimórfico o FK a appointments/etc
    provider VARCHAR(20) NOT NULL,
    external_event_id VARCHAR(255) NOT NULL,
    synced_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, crm_event_id, provider)
);

-- 3. Documents
CREATE TABLE IF NOT EXISTS documents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    uploaded_by UUID NOT NULL REFERENCES users(id),
    entity_type VARCHAR(50) NOT NULL, -- 'client', 'property', 'contract', 'lead'
    entity_id UUID NOT NULL,
    file_name VARCHAR(255) NOT NULL,
    file_size BIGINT NOT NULL,
    mime_type VARCHAR(100) NOT NULL,
    storage_path TEXT NOT NULL,
    version INTEGER NOT NULL DEFAULT 1,
    deleted_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 4. Document Access Logs
CREATE TABLE IF NOT EXISTS document_access_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    user_id UUID NOT NULL REFERENCES users(id),
    document_id UUID NOT NULL REFERENCES documents(id),
    action VARCHAR(50) NOT NULL, -- 'UPLOAD', 'DOWNLOAD', 'VIEW', 'DELETE'
    ip_address VARCHAR(45),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 5. Digital Signatures (DocuSign)
CREATE TABLE IF NOT EXISTS digital_signatures (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    document_id UUID NOT NULL REFERENCES documents(id),
    envelope_id VARCHAR(255),
    signer_name VARCHAR(255) NOT NULL,
    signer_email VARCHAR(255) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'DRAFT', -- DRAFT, SENT, VIEWED, SIGNED, DECLINED, EXPIRED, CANCELLED
    signed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_calendar_sync_crm_event ON calendar_sync_events(crm_event_id);
CREATE INDEX IF NOT EXISTS idx_documents_entity ON documents(entity_type, entity_id);
CREATE INDEX IF NOT EXISTS idx_documents_tenant ON documents(tenant_id, deleted_at);
CREATE INDEX IF NOT EXISTS idx_document_logs_doc_id ON document_access_logs(document_id);
CREATE INDEX IF NOT EXISTS idx_digital_signatures_tenant ON digital_signatures(tenant_id);
CREATE INDEX IF NOT EXISTS idx_digital_signatures_envelope ON digital_signatures(envelope_id);
