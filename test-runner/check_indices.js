const { Client } = require('pg');
require('dotenv').config({path: '../backend/.env'});

const db = new Client({ connectionString: process.env.DATABASE_URL });
db.connect()
  .then(() => db.query("SELECT tablename, indexname, indexdef FROM pg_indexes WHERE schemaname = 'public'"))
  .then(res => {
      console.log(JSON.stringify(res.rows, null, 2));
  })
  .finally(() => db.end());
