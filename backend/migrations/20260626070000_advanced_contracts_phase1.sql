-- 20260626070000_advanced_contracts_phase1.sql

-- 1. ENUMs
CREATE TYPE contract_status AS ENUM ('DRAFT', 'PENDING_SIGNATURE', 'SIGNED', 'ACTIVE', 'SUSPENDED', 'FINISHED', 'TERMINATED', 'ANNULLED');
CREATE TYPE contract_type AS ENUM ('HOUSING', 'COMMERCIAL', 'TEMPORARY', 'PROFESSIONAL');
CREATE TYPE contract_destination AS ENUM ('HABITATIONAL', 'COMMERCIAL', 'MIXED');
CREATE TYPE participant_role AS ENUM ('LANDLORD', 'TENANT', 'GUARANTOR', 'ATTORNEY', 'WITNESS');
CREATE TYPE guarantee_type AS ENUM ('PROPERTY', 'PAYSLIP', 'SURETY_BOND', 'BANK', 'MIXED', 'OTHER');

-- Mapear estados antiguos si existen
UPDATE contracts SET status = 'FINISHED' WHERE status = 'EXPIRED';

-- Modificar tabla contratos existente
ALTER TABLE contracts ALTER COLUMN status DROP DEFAULT;
ALTER TABLE contracts 
    ALTER COLUMN status TYPE contract_status USING status::contract_status,
    ALTER COLUMN status SET DEFAULT 'DRAFT'::contract_status;

ALTER TABLE contracts 
    ADD COLUMN contract_number VARCHAR(100),
    ADD COLUMN c_type contract_type DEFAULT 'HOUSING'::contract_type,
    ADD COLUMN c_destination contract_destination DEFAULT 'HABITATIONAL'::contract_destination,
    ADD COLUMN jurisdiction VARCHAR(255),
    ADD COLUMN city VARCHAR(255),
    ADD COLUMN province VARCHAR(255),
    ADD COLUMN currency VARCHAR(10) DEFAULT 'ARS',
    ADD COLUMN deposit_amount DECIMAL(12, 2) DEFAULT 0,
    ADD COLUMN commission_amount DECIMAL(12, 2) DEFAULT 0,
    ADD COLUMN fees_amount DECIMAL(12, 2) DEFAULT 0,
    ADD COLUMN taxes_payer VARCHAR(255),
    ADD COLUMN services_payer VARCHAR(255),
    ADD COLUMN observations TEXT,
    ADD COLUMN created_by UUID REFERENCES users(id),
    ADD COLUMN updated_by UUID REFERENCES users(id),
    ADD COLUMN deleted_by UUID REFERENCES users(id);

-- Función para autogenerar número de contrato único por tenant y año (Ej: CON-2026-0001)
CREATE OR REPLACE FUNCTION generate_contract_number()
RETURNS TRIGGER AS $$
DECLARE
    current_year INT;
    next_seq INT;
BEGIN
    current_year := EXTRACT(YEAR FROM CURRENT_DATE);
    
    SELECT COALESCE(MAX(CAST(SUBSTRING(contract_number FROM '-([0-9]+)$') AS INT)), 0) + 1
    INTO next_seq
    FROM contracts
    WHERE tenant_id = NEW.tenant_id 
      AND contract_number LIKE 'CON-' || current_year || '-%';

    IF NEW.contract_number IS NULL OR NEW.contract_number = '' THEN
        NEW.contract_number := 'CON-' || current_year || '-' || LPAD(next_seq::TEXT, 4, '0');
    END IF;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_generate_contract_number
BEFORE INSERT ON contracts
FOR EACH ROW
EXECUTE FUNCTION generate_contract_number();

-- Generar números para los contratos existentes que no lo tengan
DO $$ 
DECLARE 
    r RECORD;
BEGIN 
    FOR r IN SELECT id, tenant_id FROM contracts WHERE contract_number IS NULL LOOP 
        UPDATE contracts SET contract_number = 'CON-LEGACY-' || SUBSTRING(id::TEXT FROM 1 FOR 8) WHERE id = r.id;
    END LOOP; 
END $$;

ALTER TABLE contracts ADD CONSTRAINT uq_contract_number_tenant UNIQUE (tenant_id, contract_number);


-- 2. PARTICIPANTES
CREATE TABLE contract_participants (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    contract_id UUID NOT NULL REFERENCES contracts(id) ON DELETE CASCADE,
    client_id UUID NOT NULL REFERENCES clients(id),
    p_role participant_role NOT NULL,
    percentage DECIMAL(5, 2) DEFAULT 100.00,
    is_main BOOLEAN DEFAULT false,
    display_order INT DEFAULT 0,
    observations TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP WITH TIME ZONE,
    created_by UUID REFERENCES users(id),
    updated_by UUID REFERENCES users(id),
    deleted_by UUID REFERENCES users(id)
);

CREATE INDEX idx_cp_contract ON contract_participants(contract_id);
CREATE INDEX idx_cp_client ON contract_participants(client_id);
CREATE INDEX idx_cp_tenant ON contract_participants(tenant_id);

-- 3. GARANTÍAS (participant_guarantees)
CREATE TABLE participant_guarantees (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    participant_id UUID NOT NULL REFERENCES contract_participants(id) ON DELETE CASCADE,
    guarantee_type guarantee_type NOT NULL,
    status VARCHAR(50) DEFAULT 'ACTIVE',
    income_amount DECIMAL(12, 2),
    employer VARCHAR(255),
    guarantee_details TEXT,
    observations TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP WITH TIME ZONE,
    created_by UUID REFERENCES users(id),
    updated_by UUID REFERENCES users(id),
    deleted_by UUID REFERENCES users(id)
);

CREATE INDEX idx_pg_participant ON participant_guarantees(participant_id);


-- 4. TABLAS ESTRUCTURALES PARA FASES FUTURAS
CREATE TABLE contract_versions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    contract_id UUID NOT NULL REFERENCES contracts(id) ON DELETE CASCADE,
    version_number INT NOT NULL,
    changes_summary TEXT,
    contract_snapshot JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    created_by UUID REFERENCES users(id)
);
CREATE INDEX idx_cv_contract ON contract_versions(contract_id);

CREATE TABLE contract_templates (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    c_type contract_type NOT NULL,
    content TEXT,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP WITH TIME ZONE,
    created_by UUID REFERENCES users(id),
    updated_by UUID REFERENCES users(id),
    deleted_by UUID REFERENCES users(id)
);

CREATE TABLE contract_clauses (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    contract_id UUID REFERENCES contracts(id) ON DELETE CASCADE,
    template_id UUID REFERENCES contract_templates(id),
    title VARCHAR(255),
    content TEXT NOT NULL,
    display_order INT DEFAULT 0,
    is_mandatory BOOLEAN DEFAULT false,
    is_editable BOOLEAN DEFAULT true,
    variables JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP WITH TIME ZONE,
    created_by UUID REFERENCES users(id),
    updated_by UUID REFERENCES users(id),
    deleted_by UUID REFERENCES users(id)
);
CREATE INDEX idx_cc_contract ON contract_clauses(contract_id);

CREATE TABLE contract_documents (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    contract_id UUID NOT NULL REFERENCES contracts(id) ON DELETE CASCADE,
    document_type VARCHAR(100),
    file_path VARCHAR(500) NOT NULL,
    file_name VARCHAR(255),
    file_size INT,
    mime_type VARCHAR(100),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP WITH TIME ZONE,
    created_by UUID REFERENCES users(id),
    deleted_by UUID REFERENCES users(id)
);
CREATE INDEX idx_cd_contract ON contract_documents(contract_id);
