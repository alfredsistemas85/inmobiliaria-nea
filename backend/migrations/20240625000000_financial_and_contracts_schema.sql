-- 20240625000000_financial_and_contracts_schema.sql

-- Contratos
CREATE TABLE IF NOT EXISTS contracts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    property_id UUID NOT NULL REFERENCES properties(id),
    tenant_user_id UUID REFERENCES users(id),
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    rent_amount DECIMAL(12, 2) NOT NULL,
    index_type VARCHAR(50) DEFAULT 'FIXED', -- FIXED, ICL, CASA_PROPIA
    status VARCHAR(50) DEFAULT 'ACTIVE', -- ACTIVE, EXPIRED, TERMINATED
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_contracts_tenant ON contracts(tenant_id);
CREATE INDEX idx_contracts_property ON contracts(property_id);

-- Invoices (Facturas / Expensas / Liquidaciones generadas)
CREATE TABLE IF NOT EXISTS invoices (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    contract_id UUID REFERENCES contracts(id),
    amount DECIMAL(12, 2) NOT NULL,
    commission DECIMAL(12, 2) DEFAULT 0,
    status VARCHAR(50) DEFAULT 'PENDING', -- PENDING, PAID, OVERDUE, CANCELLED
    due_date DATE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_invoices_tenant_status ON invoices(tenant_id, status);

-- Payments (Registro de pagos de Invoices o Suscripciones SaaS)
CREATE TABLE IF NOT EXISTS payments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    type VARCHAR(50) NOT NULL, -- SUBSCRIPTION, RENT, EXPENSE
    status VARCHAR(50) DEFAULT 'PENDING', -- PENDING, APPROVED, REJECTED
    amount DECIMAL(12, 2) NOT NULL,
    mercado_pago_id VARCHAR(255) UNIQUE,
    invoice_id UUID REFERENCES invoices(id) ON DELETE SET NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_payments_tenant ON payments(tenant_id);
CREATE INDEX idx_payments_mp_id ON payments(mercado_pago_id);

-- Tenant Settings (Para credenciales de MP y datos bancarios)
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS mp_access_token VARCHAR(255);
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS mp_public_key VARCHAR(255);
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS bank_cbu VARCHAR(255);
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS bank_alias VARCHAR(255);
