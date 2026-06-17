-- Fase 5: Motor Avanzado de Ajustes de Alquileres e Instalments

-- 1. Enums
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'adjustment_method') THEN
        CREATE TYPE adjustment_method AS ENUM (
            'MANUAL',
            'FIXED_PERCENTAGE',
            'IPC',
            'ICL',
            'CASA_PROPIA',
            'CUSTOM'
        );
    END IF;

    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'adjustment_frequency') THEN
        CREATE TYPE adjustment_frequency AS ENUM (
            'MONTHLY',
            'BIMONTHLY',
            'QUARTERLY',
            'SEMESTER',
            'ANNUAL'
        );
    END IF;

    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'adjustment_status') THEN
        CREATE TYPE adjustment_status AS ENUM (
            'PENDING',
            'APPROVED',
            'REJECTED',
            'ROLLED_BACK',
            'PENDING_INDEX_DATA'
        );
    END IF;

    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'automation_mode') THEN
        CREATE TYPE automation_mode AS ENUM (
            'MANUAL',
            'SEMIAUTOMATIC',
            'AUTOMATIC'
        );
    END IF;

    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'installment_status') THEN
        CREATE TYPE installment_status AS ENUM (
            'PENDING',
            'PARTIALLY_PAID',
            'PAID',
            'OVERDUE',
            'CANCELLED'
        );
    END IF;
END
$$;

-- 2. Alter Table Contracts
ALTER TABLE contracts
    ADD COLUMN IF NOT EXISTS adjustment_method adjustment_method,
    ADD COLUMN IF NOT EXISTS adjustment_frequency adjustment_frequency,
    ADD COLUMN IF NOT EXISTS automation_mode automation_mode DEFAULT 'SEMIAUTOMATIC',
    ADD COLUMN IF NOT EXISTS fixed_percentage NUMERIC(10,2),
    ADD COLUMN IF NOT EXISTS first_notification_days INTEGER DEFAULT 30,
    ADD COLUMN IF NOT EXISTS second_notification_days INTEGER DEFAULT 7,
    ADD COLUMN IF NOT EXISTS third_notification_days INTEGER DEFAULT 1,
    ADD COLUMN IF NOT EXISTS requires_manual_approval BOOLEAN DEFAULT TRUE,
    ADD COLUMN IF NOT EXISTS next_adjustment_date DATE,
    ADD COLUMN IF NOT EXISTS last_adjustment_date DATE,
    ADD COLUMN IF NOT EXISTS original_rent_amount NUMERIC(15,2),
    ADD COLUMN IF NOT EXISTS current_rent_amount NUMERIC(15,2);

-- Migrar datos de rent_amount a las nuevas columnas
UPDATE contracts
SET original_rent_amount = rent_amount
WHERE original_rent_amount IS NULL;

UPDATE contracts
SET current_rent_amount = rent_amount
WHERE current_rent_amount IS NULL;

-- 3. Tabla de Historial de Ajustes
CREATE TABLE IF NOT EXISTS rent_adjustments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    contract_id UUID NOT NULL REFERENCES contracts(id) ON DELETE CASCADE,
    adjustment_method adjustment_method NOT NULL,
    status adjustment_status NOT NULL DEFAULT 'PENDING',
    previous_amount NUMERIC(15,2) NOT NULL,
    new_amount NUMERIC(15,2) NOT NULL,
    percentage_applied NUMERIC(10,4),
    index_name VARCHAR(50),
    index_initial_value NUMERIC(15,6),
    index_final_value NUMERIC(15,6),
    index_snapshot JSONB,
    rollback_reason TEXT,
    approved_by UUID REFERENCES users(id),
    approved_at TIMESTAMP WITH TIME ZONE,
    effective_date DATE NOT NULL,
    notes TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_rent_adjustments_contract ON rent_adjustments(contract_id);
CREATE INDEX IF NOT EXISTS idx_rent_adjustments_tenant ON rent_adjustments(tenant_id);

-- 4. Tabla de Cuotas (Installments)
CREATE TABLE IF NOT EXISTS contract_installments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    contract_id UUID NOT NULL REFERENCES contracts(id) ON DELETE CASCADE,
    due_date DATE NOT NULL,
    amount NUMERIC(15,2) NOT NULL,
    status installment_status NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_installments_contract ON contract_installments(contract_id);
CREATE INDEX IF NOT EXISTS idx_installments_tenant ON contract_installments(tenant_id);

-- 5. Tabla genérica de Logs de Auditoría
CREATE TABLE IF NOT EXISTS audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    contract_id UUID REFERENCES contracts(id) ON DELETE SET NULL,
    action VARCHAR(255) NOT NULL,
    old_data JSONB,
    new_data JSONB,
    method VARCHAR(50),
    ip_address VARCHAR(45),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_audit_logs_tenant ON audit_logs(tenant_id);
CREATE INDEX IF NOT EXISTS idx_audit_logs_contract ON audit_logs(contract_id);
