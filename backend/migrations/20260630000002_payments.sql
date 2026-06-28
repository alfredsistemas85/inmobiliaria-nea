-- 20260630000002_payments.sql
-- Financial Bounded Context: Payments and Cash

CREATE TABLE IF NOT EXISTS receipts (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    receipt_number VARCHAR(100),
    status VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS receipt_items (
    id UUID PRIMARY KEY,
    receipt_id UUID NOT NULL REFERENCES receipts(id),
    description TEXT NOT NULL,
    amount DECIMAL(19, 4) NOT NULL
);

CREATE TABLE IF NOT EXISTS cash_boxes (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name VARCHAR(255) NOT NULL,
    type VARCHAR(50) NOT NULL,
    currency VARCHAR(10) NOT NULL,
    opening_balance DECIMAL(19, 4) NOT NULL DEFAULT 0,
    current_balance DECIMAL(19, 4) NOT NULL DEFAULT 0,
    status VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS payments (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    account_id UUID NOT NULL,
    receipt_id UUID REFERENCES receipts(id),
    payment_method VARCHAR(50) NOT NULL,
    payment_reference VARCHAR(255),
    amount DECIMAL(19, 4) NOT NULL,
    currency VARCHAR(10) NOT NULL,
    payment_date TIMESTAMPTZ NOT NULL,
    status VARCHAR(50) NOT NULL,
    external_provider VARCHAR(100),
    external_reference VARCHAR(255),
    idempotency_key UUID UNIQUE,
    created_by UUID,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS payment_allocations (
    id UUID PRIMARY KEY,
    payment_id UUID NOT NULL REFERENCES payments(id),
    installment_id UUID NOT NULL,
    principal_amount DECIMAL(19, 4) NOT NULL DEFAULT 0,
    interest_amount DECIMAL(19, 4) NOT NULL DEFAULT 0,
    expense_amount DECIMAL(19, 4) NOT NULL DEFAULT 0,
    total_allocated DECIMAL(19, 4) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS bank_transactions (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    account_number VARCHAR(100) NOT NULL,
    transaction_date TIMESTAMPTZ NOT NULL,
    description TEXT NOT NULL,
    amount DECIMAL(19, 4) NOT NULL,
    reference VARCHAR(255),
    reconciliation_status VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS reconciliations (
    id UUID PRIMARY KEY,
    payment_id UUID REFERENCES payments(id),
    bank_transaction_id UUID REFERENCES bank_transactions(id),
    confidence DECIMAL(5, 2) NOT NULL,
    matched_by UUID,
    matched_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS payment_outbox_events (
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

CREATE TABLE IF NOT EXISTS financial_audit (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    user_id UUID,
    ip VARCHAR(45),
    user_agent TEXT,
    correlation_id VARCHAR(100),
    entity_type VARCHAR(100) NOT NULL,
    entity_id UUID NOT NULL,
    payload JSONB,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
