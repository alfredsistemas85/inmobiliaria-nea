-- Migration: 20240623000000_fase5_1_documents_metadata.sql
-- Description: Add document_type and rent_adjustment_id to documents for traceability

-- 1. Add document_type and rent_adjustment_id columns
ALTER TABLE documents
ADD COLUMN IF NOT EXISTS document_type VARCHAR(50) DEFAULT 'OTHER',
ADD COLUMN IF NOT EXISTS rent_adjustment_id UUID REFERENCES rent_adjustments(id);

-- 2. Update existing rows (optional, but safe)
UPDATE documents SET document_type = 'OTHER' WHERE document_type IS NULL;

-- 3. Make document_type NOT NULL if we are sure, but for safety in migrations we leave it nullable or with default.
ALTER TABLE documents ALTER COLUMN document_type SET NOT NULL;
