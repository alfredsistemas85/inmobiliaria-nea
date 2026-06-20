-- Eliminar la restricción NOT NULL de plan_id porque fue deprecado en la Fase 3.
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'subscriptions' AND column_name = 'plan_id') THEN
        ALTER TABLE subscriptions ALTER COLUMN plan_id DROP NOT NULL;
    END IF;
END $$;
