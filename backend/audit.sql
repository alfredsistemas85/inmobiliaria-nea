\x on

-- FASE 1
SELECT '--- FASE 1: INSPECCION DE TABLAS ---' as section;
SELECT table_name
FROM information_schema.tables
WHERE table_schema='public'
ORDER BY table_name;

-- FASE 2
SELECT '--- FASE 2: ESTRUCTURA DE TENANTS ---' as section;
SELECT column_name, data_type, is_nullable, column_default
FROM information_schema.columns
WHERE table_name='tenants'
ORDER BY ordinal_position;

-- FASE 3
SELECT '--- FASE 3: ESTRUCTURA DE SUBSCRIPTIONS ---' as section;
SELECT column_name, data_type, is_nullable, column_default
FROM information_schema.columns
WHERE table_name='subscriptions'
ORDER BY ordinal_position;

-- FASE 4
SELECT '--- FASE 4: FOREIGN KEYS ---' as section;
SELECT tc.constraint_name, tc.table_name, kcu.column_name, ccu.table_name AS foreign_table_name, ccu.column_name AS foreign_column_name
FROM information_schema.table_constraints tc
JOIN information_schema.key_column_usage kcu ON tc.constraint_name = kcu.constraint_name
JOIN information_schema.constraint_column_usage ccu ON ccu.constraint_name = tc.constraint_name
WHERE tc.constraint_type='FOREIGN KEY';

-- FASE 5
SELECT '--- FASE 5: CONSTRAINTS ---' as section;
SELECT conrelid::regclass, conname, pg_get_constraintdef(oid)
FROM pg_constraint
WHERE conrelid::regclass::text='subscriptions';

-- FASE 6
SELECT '--- FASE 6: TRIGGERS ---' as section;
SELECT trigger_name, event_manipulation, event_object_table, action_statement
FROM information_schema.triggers
WHERE trigger_schema='public';

-- FASE 7
SELECT '--- FASE 7: FUNCIONES SQL ---' as section;
SELECT routine_name
FROM information_schema.routines
WHERE routine_schema='public'
  AND (routine_name LIKE '%tenant%' OR routine_name LIKE '%subscription%' OR routine_name LIKE '%plan%');

-- FASE 8
SELECT '--- FASE 8: DATOS REALES ---' as section;
SELECT 'PLANS' as table_name;
SELECT * FROM plans LIMIT 20;

SELECT 'SUBSCRIPTIONS' as table_name;
SELECT * FROM subscriptions LIMIT 20;

SELECT 'TENANTS' as table_name;
SELECT * FROM tenants LIMIT 20;
