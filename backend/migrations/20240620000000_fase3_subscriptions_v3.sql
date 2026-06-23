-- migration_fase3_subscriptions_v3.sql
DO $$ 
DECLARE 
    tenant_count INT := 0; sub_count INT := 0; plan_count INT := 0;
    has_subscriptions BOOLEAN := FALSE; has_plans BOOLEAN := FALSE;
    has_audit BOOLEAN := FALSE; has_users BOOLEAN := FALSE;
BEGIN
    -- Auditoría previa
    IF EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name='tenants') THEN
        SELECT COUNT(*) INTO tenant_count FROM tenants; END IF;
    IF EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name='subscriptions') THEN
        has_subscriptions := TRUE; SELECT COUNT(*) INTO sub_count FROM subscriptions; END IF;
    IF EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name='plans') THEN
        has_plans := TRUE; SELECT COUNT(*) INTO plan_count FROM plans; END IF;
    IF EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name='audit_logs') THEN has_audit := TRUE; END IF;
    IF EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name='users') THEN has_users := TRUE; END IF;
    
    RAISE NOTICE 'Audit - Tenants: %, Subscriptions: %, Plans: %', tenant_count, sub_count, plan_count;
    
    -- 1. ENUMS de PostgreSQL
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'plan_type') THEN
        CREATE TYPE plan_type AS ENUM ('BASIC', 'PRO'); END IF;
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'subscription_status') THEN
        CREATE TYPE subscription_status AS ENUM ('TRIAL', 'ACTIVE', 'SUSPENDED', 'CANCELLED'); END IF;
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'user_role') THEN
        CREATE TYPE user_role AS ENUM ('SUPERADMIN', 'ADMIN_INMOBILIARIA', 'SUPERVISOR', 'AGENTE', 'OPERADOR'); END IF;

    -- 2. MIGRACION AUDITORIA (audit_logs)
    IF has_audit THEN
        IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='audit_logs' AND column_name='ip_address') THEN
            ALTER TABLE audit_logs ADD COLUMN ip_address VARCHAR(45); END IF;
        IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='audit_logs' AND column_name='user_agent') THEN
            ALTER TABLE audit_logs ADD COLUMN user_agent TEXT; END IF;
        IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='audit_logs' AND column_name='old_data') THEN
            ALTER TABLE audit_logs ADD COLUMN old_data JSONB; END IF;
        IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='audit_logs' AND column_name='new_data') THEN
            ALTER TABLE audit_logs ADD COLUMN new_data JSONB; END IF;
    END IF;

    -- 3. MIGRACION ROLES (RBAC en Users)
    IF has_users THEN
        IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='users' AND column_name='role_id') THEN
            ALTER TABLE users ADD COLUMN role user_role DEFAULT 'AGENTE';
            IF EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name='roles') THEN
                UPDATE users u SET role = CASE 
                    WHEN r.name ILIKE '%admin%' THEN 'ADMIN_INMOBILIARIA'::user_role
                    WHEN r.name ILIKE '%super%' THEN 'SUPERADMIN'::user_role
                    WHEN r.name ILIKE '%operador%' THEN 'OPERADOR'::user_role
                    WHEN r.name ILIKE '%supervisor%' THEN 'SUPERVISOR'::user_role
                    ELSE 'AGENTE'::user_role END
                FROM roles r WHERE u.role_id = r.id;
            END IF;
            ALTER TABLE users DROP COLUMN role_id;
        END IF;
        IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='users' AND column_name='onboarding_token') THEN
            ALTER TABLE users ADD COLUMN onboarding_token VARCHAR(255); END IF;
        IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='users' AND column_name='onboarding_token_expires_at') THEN
            ALTER TABLE users ADD COLUMN onboarding_token_expires_at TIMESTAMPTZ; END IF;
    END IF;

    -- 4. MIGRACION SUSCRIPCIONES
    IF has_subscriptions THEN
        IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='subscriptions' AND column_name='plan_type') THEN
            ALTER TABLE subscriptions ADD COLUMN plan_type plan_type NOT NULL DEFAULT 'BASIC'; END IF;
        IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='subscriptions' AND column_name='new_status') THEN
            ALTER TABLE subscriptions ADD COLUMN new_status subscription_status NOT NULL DEFAULT 'TRIAL'; END IF;
        IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='subscriptions' AND column_name='trial_ends_at') THEN
            ALTER TABLE subscriptions ADD COLUMN trial_ends_at TIMESTAMPTZ; END IF;
        IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='subscriptions' AND column_name='starts_at') THEN
            ALTER TABLE subscriptions ADD COLUMN starts_at TIMESTAMPTZ; END IF;
        IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='subscriptions' AND column_name='expires_at') THEN
            ALTER TABLE subscriptions ADD COLUMN expires_at TIMESTAMPTZ; END IF;

        IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='subscriptions' AND column_name='status') THEN
            UPDATE subscriptions SET new_status = CASE 
                WHEN status::text ILIKE 'active' THEN 'ACTIVE'::subscription_status
                WHEN status::text ILIKE 'canceled' THEN 'CANCELLED'::subscription_status
                WHEN status::text ILIKE 'past_due' THEN 'SUSPENDED'::subscription_status
                ELSE 'TRIAL'::subscription_status END;
            ALTER TABLE subscriptions DROP COLUMN status;
        END IF;

        ALTER TABLE subscriptions RENAME COLUMN new_status TO status;
        
        IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'uq_subscriptions_tenant') THEN
            BEGIN
                ALTER TABLE subscriptions ADD CONSTRAINT uq_subscriptions_tenant UNIQUE (tenant_id);
            EXCEPTION WHEN unique_violation THEN
                RAISE WARNING 'No se pudo crear constraint UNIQUE en tenant_id debido a datos duplicados.';
            END;
        END IF;
    ELSE
        CREATE TABLE subscriptions (
            id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
            tenant_id UUID NOT NULL UNIQUE REFERENCES tenants(id) ON DELETE CASCADE,
            plan_type plan_type NOT NULL DEFAULT 'BASIC',
            status subscription_status NOT NULL DEFAULT 'TRIAL',
            starts_at TIMESTAMPTZ, expires_at TIMESTAMPTZ, trial_ends_at TIMESTAMPTZ,
            created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
        );
    END IF;

    CREATE INDEX IF NOT EXISTS idx_subscriptions_status ON subscriptions(status);
    CREATE INDEX IF NOT EXISTS idx_subscriptions_plan ON subscriptions(plan_type);

END $$;
