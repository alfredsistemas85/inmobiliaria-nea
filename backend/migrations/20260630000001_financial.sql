-- 20260630000001_financial.sql
-- Financial Bounded Context: Accounts and Billing

CREATE TABLE IF NOT EXISTS tenant_accounts (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    status VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS customer_accounts (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    customer_id UUID NOT NULL,
    status VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS contract_accounts (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    contract_id UUID NOT NULL,
    status VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS installments (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    contract_account_id UUID NOT NULL REFERENCES contract_accounts(id),
    number INTEGER NOT NULL,
    due_date DATE NOT NULL,
    original_amount DECIMAL(19, 4) NOT NULL,
    current_amount DECIMAL(19, 4) NOT NULL,
    interest_amount DECIMAL(19, 4) NOT NULL DEFAULT 0,
    paid_amount DECIMAL(19, 4) NOT NULL DEFAULT 0,
    remaining_balance DECIMAL(19, 4) NOT NULL,
    currency VARCHAR(10) NOT NULL,
    index_value DECIMAL(19, 4),
    status VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS interest_calculations (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    installment_id UUID NOT NULL REFERENCES installments(id),
    calculated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    amount DECIMAL(19, 4) NOT NULL,
    details JSONB
);

CREATE TABLE IF NOT EXISTS financial_events (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    event_type VARCHAR(100) NOT NULL,
    aggregate_id UUID NOT NULL,
    payload JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS billing_outbox_events (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    aggregate_type VARCHAR(100) NOT NULL,
    aggregate_id UUID NOT NULL,
    event_type VARCHAR(100) NOT NULL,
    payload JSONB NOT NULL,
    processed BOOLEAN NOT NULL DEFAULT FALSE,
    processed_at TIMESTAMPTZ,
    retries INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS tenant_financial_statistics (
    tenant_id UUID PRIMARY KEY,
    total_balance DECIMAL(19, 4) NOT NULL DEFAULT 0,
    total_debt DECIMAL(19, 4) NOT NULL DEFAULT 0,
    overdue_debt DECIMAL(19, 4) NOT NULL DEFAULT 0,
    pending_installments INTEGER NOT NULL DEFAULT 0,
    paid_installments INTEGER NOT NULL DEFAULT 0,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
