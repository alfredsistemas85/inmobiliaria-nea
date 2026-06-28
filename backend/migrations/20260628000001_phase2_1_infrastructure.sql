-- 1.1 Habilitar extensiones necesarias
CREATE EXTENSION IF NOT EXISTS btree_gist;
CREATE EXTENSION IF NOT EXISTS pgcrypto;

-- 1.2 Agregar campos opcionales a contracts
ALTER TABLE contracts 
    ADD COLUMN IF NOT EXISTS snapshot_payload JSONB DEFAULT NULL,
    ADD COLUMN IF NOT EXISTS parent_contract_id UUID DEFAULT NULL;

-- Agregar Foreign Key para parent_contract_id
ALTER TABLE contracts
    DROP CONSTRAINT IF EXISTS fk_contracts_parent_contract;

ALTER TABLE contracts
    ADD CONSTRAINT fk_contracts_parent_contract 
    FOREIGN KEY (parent_contract_id) 
    REFERENCES contracts(id) 
    ON DELETE SET NULL;

-- 1.3 Crear tabla genérica de auditoría
CREATE TABLE IF NOT EXISTS system_audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    entity_type VARCHAR(100) NOT NULL,
    entity_id UUID NOT NULL,
    action VARCHAR(50) NOT NULL,
    old_payload JSONB,
    new_payload JSONB,
    actor_id UUID NOT NULL,
    ip_address VARCHAR(45),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 1.4 Crear índices para auditoría
CREATE INDEX IF NOT EXISTS idx_sysaudit_tenant ON system_audit_logs(tenant_id);
CREATE INDEX IF NOT EXISTS idx_sysaudit_entity_type ON system_audit_logs(entity_type);
CREATE INDEX IF NOT EXISTS idx_sysaudit_entity_id ON system_audit_logs(entity_id);
CREATE INDEX IF NOT EXISTS idx_sysaudit_created_at ON system_audit_logs(created_at);

-- 1.5 Constraint GiST para prevención de superposición de contratos
ALTER TABLE contracts 
    DROP CONSTRAINT IF EXISTS prevent_overlapping_contracts;

-- La constraint excluye filas donde el property_id sea igual y el rango (start_date, end_date) se solape,
-- pero SOLO aplica a los estados considerados "ocupantes" (ACTIVE y PENDING_SIGNATURE).
ALTER TABLE contracts 
    ADD CONSTRAINT prevent_overlapping_contracts 
    EXCLUDE USING gist (
        property_id WITH =,
        daterange(start_date, end_date, '[]') WITH &&
    ) WHERE (status IN ('ACTIVE', 'PENDING_SIGNATURE'));
