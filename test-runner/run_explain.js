const { Client } = require('pg');
require('dotenv').config({path: '../backend/.env'});

const queries = [
  { name: "Dashboard Clients", query: "EXPLAIN ANALYZE SELECT COUNT(*) FROM clients WHERE tenant_id = '00000000-0000-0000-0000-000000000000' AND deleted_at IS NULL" },
  { name: "Dashboard Properties", query: "EXPLAIN ANALYZE SELECT COUNT(*) FROM properties WHERE tenant_id = '00000000-0000-0000-0000-000000000000' AND deleted_at IS NULL" },
  { name: "Dashboard Leads", query: "EXPLAIN ANALYZE SELECT COUNT(*) FROM leads WHERE tenant_id = '00000000-0000-0000-0000-000000000000' AND status = 'NUEVO' AND deleted_at IS NULL" },
  { name: "Dashboard Appointments", query: "EXPLAIN ANALYZE SELECT COUNT(*) FROM appointments WHERE tenant_id = '00000000-0000-0000-0000-000000000000' AND scheduled_at >= CURRENT_TIMESTAMP AND deleted_at IS NULL" },
  { name: "Leads List", query: "EXPLAIN ANALYZE SELECT COUNT(*) FROM leads l LEFT JOIN clients c ON l.client_id = c.id WHERE l.tenant_id = '00000000-0000-0000-0000-000000000000' AND l.deleted_at IS NULL AND (c.first_name ILIKE '%juan%' OR c.last_name ILIKE '%juan%')" },
  { name: "Appointments List", query: "EXPLAIN ANALYZE SELECT COUNT(*) FROM appointments a LEFT JOIN clients c ON a.client_id = c.id WHERE a.tenant_id = '00000000-0000-0000-0000-000000000000' AND a.deleted_at IS NULL AND (a.notes ILIKE '%test%' OR c.first_name ILIKE '%test%' OR c.last_name ILIKE '%test%')" },
  { name: "WhatsApp Conversations", query: "EXPLAIN ANALYZE SELECT c.id, c.tenant_id, c.client_id, c.status, c.created_at, c.updated_at, cl.first_name, cl.last_name, cl.phone, (SELECT content FROM messages WHERE conversation_id = c.id AND deleted_at IS NULL ORDER BY created_at DESC LIMIT 1) as last_message_content FROM conversations c LEFT JOIN clients cl ON c.client_id = cl.id WHERE c.tenant_id = '00000000-0000-0000-0000-000000000000' AND c.deleted_at IS NULL ORDER BY c.updated_at DESC LIMIT 50" }
];

const db = new Client({ connectionString: process.env.DATABASE_URL });
db.connect()
  .then(async () => {
      let report = "# Reporte de EXPLAIN ANALYZE (Performance)\\n\\n";
      for (const q of queries) {
          try {
              const res = await db.query(q.query);
              report += `## ${q.name}\n\`\`\`sql\n${q.query}\n\`\`\`\n\`\`\`\n`;
              res.rows.forEach(r => {
                  report += Object.values(r)[0] + "\n";
              });
              report += `\`\`\`\n\n`;
          } catch(e) {
              console.error("Error:", q.name, e.message);
          }
      }
      require('fs').writeFileSync('../explain_analyze_report.md', report);
      console.log("Report generated successfully");
  })
  .catch(console.error)
  .finally(() => db.end());
