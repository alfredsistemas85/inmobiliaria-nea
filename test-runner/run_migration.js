const { Client } = require('pg');
const fs = require('fs');
require('dotenv').config({path: '../backend/.env'});

const db = new Client({ connectionString: process.env.DATABASE_URL });
db.connect()
  .then(() => {
      const sql = fs.readFileSync('../database/migrations/20240614030000_phase4b_commercial_ops.sql', 'utf8');
      return db.query(sql);
  })
  .then(() => console.log('Migration successful'))
  .catch(console.error)
  .finally(() => db.end());
