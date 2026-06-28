-- Add V2 fields to contracts table

-- Types might not exist if they were never created
DO $$ BEGIN
    CREATE TYPE adjustment_method AS ENUM ('MANUAL', 'FIXED_PERCENTAGE', 'IPC', 'ICL', 'CASA_PROPIA', 'CUSTOM');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE adjustment_frequency AS ENUM ('MONTHLY', 'BIMONTHLY', 'QUARTERLY', 'SEMESTER', 'ANNUAL');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE automation_mode AS ENUM ('MANUAL', 'SEMIAUTOMATIC', 'AUTOMATIC');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE contract_type AS ENUM ('HOUSING', 'COMMERCIAL', 'TEMPORARY', 'PROFESSIONAL');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE contract_destination AS ENUM ('HABITATIONAL', 'COMMERCIAL', 'MIXED');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE contract_status AS ENUM ('DRAFT', 'PENDING_SIGNATURE', 'SIGNED', 'ACTIVE', 'SUSPENDED', 'FINISHED', 'TERMINATED', 'ANNULLED');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

ALTER TABLE contracts
    ADD COLUMN IF NOT EXISTS original_rent_amount DECIMAL(12, 2),
    ADD COLUMN IF NOT EXISTS current_rent_amount DECIMAL(12, 2),
    ADD COLUMN IF NOT EXISTS adjustment_method adjustment_method,
    ADD COLUMN IF NOT EXISTS adjustment_frequency adjustment_frequency,
    ADD COLUMN IF NOT EXISTS automation_mode automation_mode,
    ADD COLUMN IF NOT EXISTS fixed_percentage DECIMAL(5, 2),
    ADD COLUMN IF NOT EXISTS first_notification_days INTEGER,
    ADD COLUMN IF NOT EXISTS second_notification_days INTEGER,
    ADD COLUMN IF NOT EXISTS third_notification_days INTEGER,
    ADD COLUMN IF NOT EXISTS requires_manual_approval BOOLEAN DEFAULT false,
    ADD COLUMN IF NOT EXISTS next_adjustment_date DATE,
    ADD COLUMN IF NOT EXISTS last_adjustment_date DATE,
    ADD COLUMN IF NOT EXISTS contract_number VARCHAR(50),
    ADD COLUMN IF NOT EXISTS c_type contract_type,
    ADD COLUMN IF NOT EXISTS c_destination contract_destination,
    ADD COLUMN IF NOT EXISTS jurisdiction VARCHAR(100),
    ADD COLUMN IF NOT EXISTS city VARCHAR(100),
    ADD COLUMN IF NOT EXISTS province VARCHAR(100),
    ADD COLUMN IF NOT EXISTS currency VARCHAR(10),
    ADD COLUMN IF NOT EXISTS deposit_amount DECIMAL(12, 2),
    ADD COLUMN IF NOT EXISTS commission_amount DECIMAL(12, 2),
    ADD COLUMN IF NOT EXISTS fees_amount DECIMAL(12, 2),
    ADD COLUMN IF NOT EXISTS taxes_payer VARCHAR(255),
    ADD COLUMN IF NOT EXISTS services_payer VARCHAR(255),
    ADD COLUMN IF NOT EXISTS observations TEXT,
    ADD COLUMN IF NOT EXISTS template_id UUID REFERENCES contract_templates(id) ON DELETE SET NULL,
    ADD COLUMN IF NOT EXISTS created_by UUID,
    ADD COLUMN IF NOT EXISTS updated_by UUID,
    ADD COLUMN IF NOT EXISTS deleted_by UUID,
    ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMP WITH TIME ZONE;

-- Modificar el tipo de status de varchar a contract_status si se puede (esto puede requerir cast o borrar valor default temporalmente)
-- Al parecer, el model rust tiene #[sqlx(type_name = "contract_status")], pero si en la base es VARCHAR, fallará.
-- Para prevenir que falle:
ALTER TABLE contracts ALTER COLUMN status DROP DEFAULT;
ALTER TABLE contracts ALTER COLUMN status TYPE contract_status USING status::contract_status;
ALTER TABLE contracts ALTER COLUMN status SET DEFAULT 'ACTIVE'::contract_status;
