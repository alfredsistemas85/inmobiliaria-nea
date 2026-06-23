-- Add owner_id to properties table
ALTER TABLE properties ADD COLUMN IF NOT EXISTS owner_id UUID REFERENCES clients(id) ON DELETE SET NULL;
CREATE INDEX IF NOT EXISTS idx_properties_owner_id ON properties(owner_id);

-- Add client_id to contracts table
ALTER TABLE contracts ADD COLUMN IF NOT EXISTS client_id UUID REFERENCES clients(id) ON DELETE SET NULL;
CREATE INDEX IF NOT EXISTS idx_contracts_client_id ON contracts(client_id);
