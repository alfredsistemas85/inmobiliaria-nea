-- 20260627000000_contract_signatures.sql

DO $$ BEGIN
    CREATE TYPE signature_type_enum AS ENUM ('HANDDRAWN', 'DIGITAL_CERTIFICATE', 'OTP', 'BIOMETRIC');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE signature_request_status AS ENUM ('PENDING', 'OPENED', 'VIEWED', 'SIGNED', 'REJECTED', 'EXPIRED', 'CANCELLED');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- Expandimos el enum contract_status actual (si es un enum). Dado que no estamos 100% seguros de si es un ENUM en la BD actual o un VARCHAR, 
-- verificaremos y, si no existe el tipo ENUM, asumiremos que es un VARCHAR y no fallará. Si es un enum, lo modificamos.
DO $$ BEGIN
    ALTER TYPE contract_status ADD VALUE IF NOT EXISTS 'DRAFT';
    ALTER TYPE contract_status ADD VALUE IF NOT EXISTS 'READY_FOR_SIGNATURE';
    ALTER TYPE contract_status ADD VALUE IF NOT EXISTS 'SIGNING';
    ALTER TYPE contract_status ADD VALUE IF NOT EXISTS 'FINISHED';
    ALTER TYPE contract_status ADD VALUE IF NOT EXISTS 'ARCHIVED';
EXCEPTION
    WHEN duplicate_object THEN null;
    WHEN undefined_object THEN null;
END $$;


-- Tabla para versionar documentos del contrato (Original, PDFs firmados parciales/finales, anexos)
CREATE TABLE IF NOT EXISTS contract_documents (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    contract_id UUID NOT NULL REFERENCES contracts(id) ON DELETE CASCADE,
    document_type VARCHAR(50) NOT NULL, -- e.g. 'ORIGINAL', 'FINAL_SIGNED', 'ANNEX'
    storage_path VARCHAR(255) NOT NULL,
    mime_type VARCHAR(100) DEFAULT 'application/pdf',
    file_size BIGINT,
    sha256 VARCHAR(64),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP WITH TIME ZONE,
    created_by UUID,
    updated_by UUID,
    deleted_by UUID
);
CREATE INDEX IF NOT EXISTS idx_contract_documents_contract ON contract_documents(contract_id);

-- Tabla de Snapshot del contrato pre-firma
CREATE TABLE IF NOT EXISTS contract_snapshots (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    contract_id UUID NOT NULL REFERENCES contracts(id) ON DELETE CASCADE,
    snapshot_json JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_contract_snapshots_contract ON contract_snapshots(contract_id);

-- Solicitudes de firma
CREATE TABLE IF NOT EXISTS contract_signature_requests (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    contract_id UUID NOT NULL REFERENCES contracts(id) ON DELETE CASCADE,
    participant_id UUID NOT NULL REFERENCES contract_participants(id) ON DELETE CASCADE,
    token_hash VARCHAR(255) UNIQUE NOT NULL,
    verification_code VARCHAR(50) UNIQUE NOT NULL,
    signature_order INTEGER DEFAULT 1,
    required_signature BOOLEAN DEFAULT true,
    signature_type signature_type_enum DEFAULT 'HANDDRAWN',
    status signature_request_status DEFAULT 'PENDING',
    expires_at TIMESTAMP WITH TIME ZONE,
    opened_at TIMESTAMP WITH TIME ZONE,
    viewed_at TIMESTAMP WITH TIME ZONE,
    signed_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP WITH TIME ZONE,
    created_by UUID,
    updated_by UUID,
    deleted_by UUID
);
CREATE INDEX IF NOT EXISTS idx_sig_requests_contract ON contract_signature_requests(contract_id);
CREATE INDEX IF NOT EXISTS idx_sig_requests_participant ON contract_signature_requests(participant_id);
CREATE INDEX IF NOT EXISTS idx_sig_requests_token_hash ON contract_signature_requests(token_hash);

-- Firmas concretadas
CREATE TABLE IF NOT EXISTS contract_signatures (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    contract_id UUID NOT NULL REFERENCES contracts(id) ON DELETE CASCADE,
    participant_id UUID NOT NULL REFERENCES contract_participants(id) ON DELETE CASCADE,
    request_id UUID NOT NULL REFERENCES contract_signature_requests(id) ON DELETE CASCADE,
    signature_image_path VARCHAR(255),
    signature_sha256 VARCHAR(64),
    pdf_sha256 VARCHAR(64),
    signed_pdf_path VARCHAR(255),
    browser VARCHAR(255),
    operating_system VARCHAR(255),
    ip VARCHAR(45),
    user_agent TEXT,
    latitude DECIMAL(10, 8),
    longitude DECIMAL(11, 8),
    signed_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_signatures_request ON contract_signatures(request_id);

-- Auditoría de Eventos
CREATE TABLE IF NOT EXISTS contract_signature_events (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    request_id UUID NOT NULL REFERENCES contract_signature_requests(id) ON DELETE CASCADE,
    event_type VARCHAR(100) NOT NULL, -- e.g. REQUEST_CREATED, WHATSAPP_SENT
    description TEXT,
    metadata JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    created_by UUID
);
CREATE INDEX IF NOT EXISTS idx_sig_events_request ON contract_signature_events(request_id);

-- Sesiones de firma
CREATE TABLE IF NOT EXISTS contract_signature_sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    request_id UUID NOT NULL REFERENCES contract_signature_requests(id) ON DELETE CASCADE,
    opened_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    closed_at TIMESTAMP WITH TIME ZONE,
    browser VARCHAR(255),
    os VARCHAR(255),
    ip VARCHAR(45),
    user_agent TEXT,
    fingerprint VARCHAR(255),
    duration_seconds INTEGER,
    attempts INTEGER DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_sig_sessions_request ON contract_signature_sessions(request_id);
