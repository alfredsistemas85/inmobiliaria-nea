-- 20260630000004_financial_integrations.sql
-- Financial ERP: External Integrations, AFIP, MP, Webhooks

CREATE TABLE IF NOT EXISTS payment_provider_accounts (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    provider VARCHAR(50) NOT NULL,
    credentials JSONB NOT NULL,
    status VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS payment_provider_transactions (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    provider VARCHAR(50) NOT NULL,
    external_id VARCHAR(255) NOT NULL,
    payment_id UUID,
    status VARCHAR(50) NOT NULL,
    payload JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS electronic_invoices (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    receipt_id UUID NOT NULL,
    invoice_type VARCHAR(50) NOT NULL,
    point_of_sale INTEGER NOT NULL,
    invoice_number INTEGER NOT NULL,
    cae VARCHAR(100),
    cae_expiration DATE,
    status VARCHAR(50) NOT NULL,
    request_payload JSONB,
    response_payload JSONB,
    pdf_path VARCHAR(255),
    xml_path VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS webhook_events (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    provider VARCHAR(50) NOT NULL,
    event VARCHAR(100) NOT NULL,
    payload JSONB NOT NULL,
    processed BOOLEAN NOT NULL DEFAULT FALSE,
    processed_at TIMESTAMPTZ,
    retries INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS autopay_subscriptions (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    customer_id UUID NOT NULL,
    provider VARCHAR(50) NOT NULL,
    token VARCHAR(255) NOT NULL,
    status VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS notification_queue (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    installment_id UUID,
    channel VARCHAR(50) NOT NULL,
    scheduled_at TIMESTAMPTZ NOT NULL,
    sent_at TIMESTAMPTZ,
    status VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
