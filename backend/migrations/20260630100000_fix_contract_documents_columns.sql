-- 20260630100000_fix_contract_documents_columns.sql
-- Fix: la tabla contract_documents fue creada en advanced_contracts_phase1 con columna
-- `file_path`, pero el código de signatures espera `storage_path`, `sha256` y `created_by`.
-- Esta migración añade las columnas faltantes de forma idempotente.

ALTER TABLE contract_documents
    ADD COLUMN IF NOT EXISTS storage_path VARCHAR(255),
    ADD COLUMN IF NOT EXISTS sha256 VARCHAR(64),
    ADD COLUMN IF NOT EXISTS updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    ADD COLUMN IF NOT EXISTS updated_by UUID,
    ADD COLUMN IF NOT EXISTS deleted_by UUID,
    ADD COLUMN IF NOT EXISTS created_by UUID;

ALTER TABLE contract_documents ALTER COLUMN file_path DROP NOT NULL;

-- Copiar datos existentes de file_path -> storage_path para no perder información
UPDATE contract_documents
SET storage_path = file_path
WHERE storage_path IS NULL AND file_path IS NOT NULL;
