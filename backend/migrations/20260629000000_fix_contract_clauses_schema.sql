-- Fix schema mismatch for contract_clauses

DO $$ 
BEGIN
    -- Rename content to body if it exists
    IF EXISTS (
        SELECT 1 
        FROM information_schema.columns 
        WHERE table_name = 'contract_clauses' AND column_name = 'content'
    ) THEN
        ALTER TABLE contract_clauses RENAME COLUMN content TO body;
    END IF;

    -- Add is_system if it does not exist
    IF NOT EXISTS (
        SELECT 1 
        FROM information_schema.columns 
        WHERE table_name = 'contract_clauses' AND column_name = 'is_system'
    ) THEN
        ALTER TABLE contract_clauses ADD COLUMN is_system BOOLEAN DEFAULT false;
    END IF;
END $$;
