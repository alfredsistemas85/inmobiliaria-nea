-- Add deleted_at to contracts table for soft-delete support
ALTER TABLE contracts ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMPTZ;
CREATE INDEX IF NOT EXISTS idx_contracts_deleted_at ON contracts(deleted_at);
