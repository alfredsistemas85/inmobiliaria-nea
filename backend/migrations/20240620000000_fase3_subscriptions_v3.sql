-- Migration: 20240620000000_fase3_subscriptions_v3.sql
-- Description: Implement plans enum, subscription status, and role enum without dropping RBAC tables.

DO $$ 
BEGIN
    -- 1. Create Enums if they don't exist
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'plan_type') THEN
        CREATE TYPE plan_type AS ENUM ('BASIC', 'PRO');
    END IF;

    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'subscription_status') THEN
        CREATE TYPE subscription_status AS ENUM ('TRIAL', 'ACTIVE', 'SUSPENDED', 'CANCELLED');
    END IF;

    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'user_role') THEN
        CREATE TYPE user_role AS ENUM ('SUPERADMIN', 'ADMIN_INMOBILIARIA', 'SUPERVISOR', 'AGENTE', 'OPERADOR');
    END IF;
END $$;

-- 2. Modify Subscriptions Table Idempotently
DO $$ 
BEGIN
    -- Only modify if subscriptions table exists
    IF EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'subscriptions') THEN
        
        -- Add plan_type if not exists
        IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'subscriptions' AND column_name = 'plan_type') THEN
            ALTER TABLE subscriptions ADD COLUMN plan_type plan_type NOT NULL DEFAULT 'BASIC';
        END IF;

        -- Add status if not exists
        IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'subscriptions' AND column_name = 'status' AND data_type = 'USER-DEFINED' AND udt_name = 'subscription_status') THEN
            -- If status exists but is varchar, alter type
            IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'subscriptions' AND column_name = 'status' AND data_type = 'character varying') THEN
                ALTER TABLE subscriptions ALTER COLUMN status DROP DEFAULT;
                ALTER TABLE subscriptions ALTER COLUMN status TYPE subscription_status USING upper(status)::subscription_status;
                ALTER TABLE subscriptions ALTER COLUMN status SET DEFAULT 'TRIAL'::subscription_status;
            ELSE
                ALTER TABLE subscriptions ADD COLUMN status subscription_status NOT NULL DEFAULT 'TRIAL';
            END IF;
        END IF;

        -- Add trial_ends_at if not exists
        IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'subscriptions' AND column_name = 'trial_ends_at') THEN
            ALTER TABLE subscriptions ADD COLUMN trial_ends_at TIMESTAMPTZ;
        END IF;
        
        -- We do NOT drop plan_id to keep the data safe, just mark it as deprecated in application logic

    END IF;
END $$;

-- 3. Modify Users Table Idempotently
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'users') THEN
        
        -- Add onboarding_token
        IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'users' AND column_name = 'onboarding_token') THEN
            ALTER TABLE users ADD COLUMN onboarding_token VARCHAR(255);
        END IF;

        -- Add onboarding_token_expires_at
        IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'users' AND column_name = 'onboarding_token_expires_at') THEN
            ALTER TABLE users ADD COLUMN onboarding_token_expires_at TIMESTAMPTZ;
        END IF;

        -- Add role ENUM
        IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'users' AND column_name = 'role') THEN
            ALTER TABLE users ADD COLUMN role user_role;
            
            -- Optional: Migrate data from roles table if role_id exists
            IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'users' AND column_name = 'role_id') THEN
                UPDATE users SET role = 
                    CASE r.name
                        WHEN 'super_admin' THEN 'SUPERADMIN'::user_role
                        WHEN 'tenant_admin' THEN 'ADMIN_INMOBILIARIA'::user_role
                        WHEN 'tenant_agent' THEN 'AGENTE'::user_role
                        ELSE 'AGENTE'::user_role
                    END
                FROM roles r WHERE users.role_id = r.id;
            END IF;
            
            -- Enforce NOT NULL after migration
            ALTER TABLE users ALTER COLUMN role SET NOT NULL DEFAULT 'AGENTE'::user_role;
        END IF;
        
        -- Do NOT drop role_id to preserve foreign key integrity just in case.

    END IF;
END $$;

-- 4. Modify Audit Logs Idempotently
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'audit_logs') THEN
        
        IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'audit_logs' AND column_name = 'ip_address') THEN
            ALTER TABLE audit_logs ADD COLUMN ip_address VARCHAR(45);
        END IF;

        IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'audit_logs' AND column_name = 'user_agent') THEN
            ALTER TABLE audit_logs ADD COLUMN user_agent TEXT;
        END IF;

        IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'audit_logs' AND column_name = 'old_data') THEN
            ALTER TABLE audit_logs ADD COLUMN old_data JSONB;
        END IF;

        IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'audit_logs' AND column_name = 'new_data') THEN
            ALTER TABLE audit_logs ADD COLUMN new_data JSONB;
        END IF;

    END IF;
END $$;
