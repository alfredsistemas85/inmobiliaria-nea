-- 20260630000003_financial_erp.sql
-- Financial ERP: Owner Accounting, Treasury and General Ledger

CREATE TABLE IF NOT EXISTS owner_statements (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    owner_id UUID NOT NULL,
    contract_id UUID NOT NULL,
    period VARCHAR(50) NOT NULL,
    gross_income DECIMAL(19, 4) NOT NULL DEFAULT 0,
    commission_amount DECIMAL(19, 4) NOT NULL DEFAULT 0,
    expenses_amount DECIMAL(19, 4) NOT NULL DEFAULT 0,
    taxes_amount DECIMAL(19, 4) NOT NULL DEFAULT 0,
    net_amount DECIMAL(19, 4) NOT NULL DEFAULT 0,
    status VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    approved_at TIMESTAMPTZ,
    paid_at TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS owner_statement_items (
    id UUID PRIMARY KEY,
    statement_id UUID NOT NULL REFERENCES owner_statements(id),
    item_type VARCHAR(50) NOT NULL,
    amount DECIMAL(19, 4) NOT NULL,
    description TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS owner_payments (
    id UUID PRIMARY KEY,
    statement_id UUID NOT NULL REFERENCES owner_statements(id),
    payment_method VARCHAR(50) NOT NULL,
    amount DECIMAL(19, 4) NOT NULL,
    payment_date TIMESTAMPTZ NOT NULL,
    reference VARCHAR(255),
    status VARCHAR(50) NOT NULL
);

CREATE TABLE IF NOT EXISTS treasury_accounts (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name VARCHAR(255) NOT NULL,
    type VARCHAR(50) NOT NULL,
    currency VARCHAR(10) NOT NULL,
    current_balance DECIMAL(19, 4) NOT NULL DEFAULT 0,
    status VARCHAR(50) NOT NULL
);

CREATE TABLE IF NOT EXISTS treasury_movements (
    id UUID PRIMARY KEY,
    account_id UUID NOT NULL REFERENCES treasury_accounts(id),
    movement_type VARCHAR(50) NOT NULL,
    amount DECIMAL(19, 4) NOT NULL,
    reference VARCHAR(255),
    description TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS chart_of_accounts (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    code VARCHAR(50) NOT NULL,
    name VARCHAR(255) NOT NULL,
    account_type VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS journal_entries (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    entry_date DATE NOT NULL,
    description TEXT NOT NULL,
    status VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS journal_entry_lines (
    id UUID PRIMARY KEY,
    entry_id UUID NOT NULL REFERENCES journal_entries(id),
    account_id UUID NOT NULL REFERENCES chart_of_accounts(id),
    debit DECIMAL(19, 4) NOT NULL DEFAULT 0,
    credit DECIMAL(19, 4) NOT NULL DEFAULT 0,
    cost_center VARCHAR(100),
    reference VARCHAR(255)
);

CREATE TABLE IF NOT EXISTS accounting_periods (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    month INTEGER NOT NULL,
    year INTEGER NOT NULL,
    status VARCHAR(50) NOT NULL,
    opened_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    closed_at TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS accounting_events (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    event_type VARCHAR(100) NOT NULL,
    aggregate_id UUID NOT NULL,
    payload JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS accounting_outbox_events (
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
