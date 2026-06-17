-- Migration: 20240624000000_fase5_2_scheduler_runs.sql
-- Description: Tracking and Idempotency for Rental Adjustment Scheduler

-- 1. Scheduler Runs tracking table
CREATE TABLE IF NOT EXISTS scheduler_runs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    process_name VARCHAR(100) NOT NULL,
    execution_date DATE NOT NULL,
    executed_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(process_name, execution_date)
);

-- 2. Prevent duplicate adjustment proposals for the same contract and date
CREATE UNIQUE INDEX IF NOT EXISTS idx_rent_adjustments_unique_proposal 
ON rent_adjustments(contract_id, effective_date) 
WHERE status <> 'REJECTED';
