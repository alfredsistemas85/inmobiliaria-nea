const { Client } = require('pg');
require('dotenv').config({path: '../backend/.env'});

const queries = [
  "CREATE INDEX IF NOT EXISTS idx_leads_tenant_status ON leads(tenant_id, status);",
  "CREATE INDEX IF NOT EXISTS idx_appointments_tenant_scheduled ON appointments(tenant_id, scheduled_at);",
  "CREATE INDEX IF NOT EXISTS idx_clients_tenant_name ON clients(tenant_id, first_name, last_name);",
  "CREATE INDEX IF NOT EXISTS idx_properties_tenant_status ON properties(tenant_id, status);"
];

const db = new Client({ connectionString: process.env.DATABASE_URL });
db.connect()
  .then(async () => {
      for (const q of queries) {
          try {
              await db.query(q);
              console.log("Success:", q);
          } catch(e) {
              console.error("Error:", q, e.message);
          }
      }
  })
  .catch(console.error)
  .finally(() => db.end());
