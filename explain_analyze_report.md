# Reporte de EXPLAIN ANALYZE (Performance)\n\n## Dashboard Clients
```sql
EXPLAIN ANALYZE SELECT COUNT(*) FROM clients WHERE tenant_id = '00000000-0000-0000-0000-000000000000' AND deleted_at IS NULL
```
```
Aggregate  (cost=1.04..1.05 rows=1 width=8) (actual time=0.055..0.055 rows=1 loops=1)
  ->  Seq Scan on clients  (cost=0.00..1.04 rows=1 width=0) (actual time=0.052..0.053 rows=0 loops=1)
        Filter: ((deleted_at IS NULL) AND (tenant_id = '00000000-0000-0000-0000-000000000000'::uuid))
        Rows Removed by Filter: 3
Planning Time: 0.121 ms
Execution Time: 0.098 ms
```

## Dashboard Properties
```sql
EXPLAIN ANALYZE SELECT COUNT(*) FROM properties WHERE tenant_id = '00000000-0000-0000-0000-000000000000' AND deleted_at IS NULL
```
```
Aggregate  (cost=2.36..2.37 rows=1 width=8) (actual time=0.007..0.007 rows=1 loops=1)
  ->  Index Scan using idx_properties_tenant_status on properties  (cost=0.14..2.36 rows=1 width=0) (actual time=0.004..0.004 rows=0 loops=1)
        Index Cond: (tenant_id = '00000000-0000-0000-0000-000000000000'::uuid)
        Filter: (deleted_at IS NULL)
Planning Time: 0.099 ms
Execution Time: 0.039 ms
```

## Dashboard Leads
```sql
EXPLAIN ANALYZE SELECT COUNT(*) FROM leads WHERE tenant_id = '00000000-0000-0000-0000-000000000000' AND status = 'NUEVO' AND deleted_at IS NULL
```
```
Aggregate  (cost=1.05..1.06 rows=1 width=8) (actual time=0.072..0.073 rows=1 loops=1)
  ->  Seq Scan on leads  (cost=0.00..1.04 rows=1 width=0) (actual time=0.070..0.070 rows=0 loops=1)
        Filter: ((deleted_at IS NULL) AND (tenant_id = '00000000-0000-0000-0000-000000000000'::uuid) AND ((status)::text = 'NUEVO'::text))
        Rows Removed by Filter: 3
Planning Time: 2.870 ms
Execution Time: 0.109 ms
```

## Dashboard Appointments
```sql
EXPLAIN ANALYZE SELECT COUNT(*) FROM appointments WHERE tenant_id = '00000000-0000-0000-0000-000000000000' AND scheduled_at >= CURRENT_TIMESTAMP AND deleted_at IS NULL
```
```
Aggregate  (cost=2.37..2.38 rows=1 width=8) (actual time=0.007..0.007 rows=1 loops=1)
  ->  Index Scan using idx_appointments_tenant on appointments  (cost=0.15..2.37 rows=1 width=0) (actual time=0.004..0.004 rows=0 loops=1)
        Index Cond: (tenant_id = '00000000-0000-0000-0000-000000000000'::uuid)
        Filter: ((deleted_at IS NULL) AND (scheduled_at >= CURRENT_TIMESTAMP))
Planning Time: 0.147 ms
Execution Time: 0.037 ms
```

## Leads List
```sql
EXPLAIN ANALYZE SELECT COUNT(*) FROM leads l LEFT JOIN clients c ON l.client_id = c.id WHERE l.tenant_id = '00000000-0000-0000-0000-000000000000' AND l.deleted_at IS NULL AND (c.first_name ILIKE '%juan%' OR c.last_name ILIKE '%juan%')
```
```
Aggregate  (cost=2.10..2.11 rows=1 width=8) (actual time=0.021..0.022 rows=1 loops=1)
  ->  Nested Loop  (cost=0.00..2.10 rows=1 width=0) (actual time=0.018..0.019 rows=0 loops=1)
        Join Filter: (c.id = l.client_id)
        ->  Seq Scan on leads l  (cost=0.00..1.04 rows=1 width=16) (actual time=0.018..0.018 rows=0 loops=1)
              Filter: ((deleted_at IS NULL) AND (tenant_id = '00000000-0000-0000-0000-000000000000'::uuid))
              Rows Removed by Filter: 3
        ->  Seq Scan on clients c  (cost=0.00..1.04 rows=1 width=16) (never executed)
              Filter: (((first_name)::text ~~* '%juan%'::text) OR ((last_name)::text ~~* '%juan%'::text))
Planning Time: 5.042 ms
Execution Time: 0.064 ms
```

## Appointments List
```sql
EXPLAIN ANALYZE SELECT COUNT(*) FROM appointments a LEFT JOIN clients c ON a.client_id = c.id WHERE a.tenant_id = '00000000-0000-0000-0000-000000000000' AND a.deleted_at IS NULL AND (a.notes ILIKE '%test%' OR c.first_name ILIKE '%test%' OR c.last_name ILIKE '%test%')
```
```
Aggregate  (cost=3.46..3.47 rows=1 width=8) (actual time=0.008..0.008 rows=1 loops=1)
  ->  Nested Loop Left Join  (cost=0.15..3.46 rows=1 width=0) (actual time=0.004..0.004 rows=0 loops=1)
        Join Filter: (a.client_id = c.id)
        Filter: ((a.notes ~~* '%test%'::text) OR ((c.first_name)::text ~~* '%test%'::text) OR ((c.last_name)::text ~~* '%test%'::text))
        ->  Index Scan using idx_appointments_tenant_scheduled on appointments a  (cost=0.15..2.37 rows=1 width=48) (actual time=0.003..0.004 rows=0 loops=1)
              Index Cond: (tenant_id = '00000000-0000-0000-0000-000000000000'::uuid)
              Filter: (deleted_at IS NULL)
        ->  Seq Scan on clients c  (cost=0.00..1.03 rows=3 width=452) (never executed)
Planning Time: 0.189 ms
Execution Time: 0.047 ms
```

## WhatsApp Conversations
```sql
EXPLAIN ANALYZE SELECT c.id, c.tenant_id, c.client_id, c.status, c.created_at, c.updated_at, cl.first_name, cl.last_name, cl.phone, (SELECT content FROM messages WHERE conversation_id = c.id AND deleted_at IS NULL ORDER BY created_at DESC LIMIT 1) as last_message_content FROM conversations c LEFT JOIN clients cl ON c.client_id = cl.id WHERE c.tenant_id = '00000000-0000-0000-0000-000000000000' AND c.deleted_at IS NULL ORDER BY c.updated_at DESC LIMIT 50
```
```
Limit  (cost=4.48..75.02 rows=1 width=768) (actual time=0.153..0.154 rows=0 loops=1)
  ->  Result  (cost=4.48..75.02 rows=1 width=768) (actual time=0.152..0.153 rows=0 loops=1)
        ->  Sort  (cost=4.48..4.48 rows=1 width=736) (actual time=0.151..0.152 rows=0 loops=1)
              Sort Key: c.updated_at DESC
              Sort Method: quicksort  Memory: 25kB
              ->  Nested Loop Left Join  (cost=1.26..4.47 rows=1 width=736) (actual time=0.099..0.100 rows=0 loops=1)
                    Join Filter: (c.client_id = cl.id)
                    ->  Bitmap Heap Scan on conversations c  (cost=1.26..3.40 rows=1 width=182) (actual time=0.098..0.099 rows=0 loops=1)
                          Recheck Cond: (deleted_at IS NULL)
                          Filter: (tenant_id = '00000000-0000-0000-0000-000000000000'::uuid)
                          Rows Removed by Filter: 3
                          Heap Blocks: exact=1
                          ->  Bitmap Index Scan on idx_conversations_deleted_at  (cost=0.00..1.26 rows=2 width=0) (actual time=0.054..0.054 rows=6 loops=1)
                                Index Cond: (deleted_at IS NULL)
                    ->  Seq Scan on clients cl  (cost=0.00..1.03 rows=3 width=570) (never executed)
        SubPlan 1
          ->  Limit  (cost=70.53..70.53 rows=1 width=16) (never executed)
                ->  Sort  (cost=70.53..74.70 rows=1668 width=16) (never executed)
                      Sort Key: messages.created_at DESC
                      ->  Index Scan using idx_messages_conversation on messages  (cost=0.28..62.19 rows=1668 width=16) (never executed)
                            Index Cond: (conversation_id = c.id)
                            Filter: (deleted_at IS NULL)
Planning Time: 0.284 ms
Execution Time: 0.200 ms
```

