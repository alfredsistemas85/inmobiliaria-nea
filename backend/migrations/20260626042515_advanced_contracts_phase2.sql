-- Fase 2: Contratos Jurídicos (Condiciones y Cláusulas)

-- 1. Plantillas de Contratos
CREATE TABLE IF NOT EXISTS contract_templates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    c_type contract_type NOT NULL, -- Enum created in Phase 1
    c_destination contract_destination NOT NULL, -- Enum created in Phase 1
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID,
    updated_by UUID,
    deleted_at TIMESTAMPTZ,
    deleted_by UUID
);
CREATE INDEX idx_contract_templates_tenant ON contract_templates(tenant_id);

-- 2. Cláusulas de Plantillas
CREATE TABLE IF NOT EXISTS template_clauses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    template_id UUID NOT NULL REFERENCES contract_templates(id) ON DELETE CASCADE,
    display_order INTEGER NOT NULL,
    title VARCHAR(255) NOT NULL,
    body TEXT NOT NULL,
    is_mandatory BOOLEAN NOT NULL DEFAULT FALSE,
    is_editable BOOLEAN NOT NULL DEFAULT TRUE,
    is_system BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID,
    updated_by UUID,
    deleted_at TIMESTAMPTZ,
    deleted_by UUID
);
CREATE INDEX idx_template_clauses_template ON template_clauses(template_id);
CREATE INDEX idx_template_clauses_tenant ON template_clauses(tenant_id);

-- 3. Términos / Condiciones Específicas del Contrato
CREATE TABLE IF NOT EXISTS contract_terms (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    contract_id UUID NOT NULL UNIQUE REFERENCES contracts(id) ON DELETE CASCADE,
    
    -- Condiciones booleanas
    allows_pets BOOLEAN NOT NULL DEFAULT FALSE,
    allows_sublease BOOLEAN NOT NULL DEFAULT FALSE,
    requires_inventory BOOLEAN NOT NULL DEFAULT FALSE,
    requires_insurance BOOLEAN NOT NULL DEFAULT FALSE,
    automatic_renewal BOOLEAN NOT NULL DEFAULT FALSE,
    
    -- Detalles
    permitted_activity VARCHAR(255),
    notice_days INTEGER,
    early_termination_penalty TEXT,
    observations TEXT,
    
    -- Auditoría
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID,
    updated_by UUID,
    deleted_at TIMESTAMPTZ,
    deleted_by UUID
);
CREATE INDEX idx_contract_terms_tenant ON contract_terms(tenant_id);

-- 4. Cláusulas Instanciadas del Contrato (Copia desde la plantilla o creadas ad-hoc)
CREATE TABLE IF NOT EXISTS contract_clauses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    contract_id UUID NOT NULL REFERENCES contracts(id) ON DELETE CASCADE,
    display_order INTEGER NOT NULL,
    title VARCHAR(255) NOT NULL,
    body TEXT NOT NULL,
    is_mandatory BOOLEAN NOT NULL DEFAULT FALSE,
    is_editable BOOLEAN NOT NULL DEFAULT TRUE,
    is_system BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID,
    updated_by UUID,
    deleted_at TIMESTAMPTZ,
    deleted_by UUID
);
CREATE INDEX idx_contract_clauses_contract ON contract_clauses(contract_id);
CREATE INDEX idx_contract_clauses_tenant ON contract_clauses(tenant_id);

-- Agregar relación opcional a contracts para saber qué plantilla se usó
ALTER TABLE contracts ADD COLUMN template_id UUID REFERENCES contract_templates(id) ON DELETE SET NULL;
