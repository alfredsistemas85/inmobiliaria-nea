-- Agregar owner_id a properties
ALTER TABLE properties ADD COLUMN IF NOT EXISTS owner_id UUID REFERENCES clients(id);

-- Agregar client_id a contracts
ALTER TABLE contracts ADD COLUMN IF NOT EXISTS client_id UUID REFERENCES clients(id);
