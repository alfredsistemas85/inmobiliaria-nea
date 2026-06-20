const { Client } = require('pg');
const fs = require('fs');
require('dotenv').config({ path: '../backend/.env' });

const API_URL = 'http://localhost:3000/api';
let authHeadersTenantA = {};
let authHeadersTenantB = {};
let authHeadersSuperAdmin = {};
let logOutput = "";

function log(msg) {
    console.log(msg);
    logOutput += msg + "\n";
}

async function queryDB(client, sql, params = []) {
    log(`[SQL] ${sql} ${params.length ? JSON.stringify(params) : ''}`);
    const res = await client.query(sql, params);
    log(`[SQL Result] ${res.rowCount} rows`);
    return res.rows;
}

async function request(method, path, body, headers = {}) {
    log(`[HTTP ${method}] ${API_URL}${path}`);
    if (body) log(`[HTTP Body] ${JSON.stringify(body)}`);
    try {
        const response = await fetch(`${API_URL}${path}`, {
            method,
            headers: { 'Content-Type': 'application/json', ...headers },
            body: body ? JSON.stringify(body) : undefined
        });
        const status = response.status;
        const text = await response.text();
        log(`[HTTP Response Status] ${status}`);
        let json;
        try { json = JSON.parse(text); log(`[HTTP Response JSON] ${text}`); } 
        catch (e) { log(`[HTTP Response Text] ${text}`); }
        return { status, json: json || text };
    } catch (e) {
        log(`[HTTP Error] ${e.message}`);
        return { status: 500, json: { error: e.message } };
    }
}

async function runAudit() {
    log("========================================");
    log("INICIANDO AUDITORIA FASE 2");
    log("========================================");
    
    // Connect to DB
    const db = new Client({ connectionString: process.env.DATABASE_URL });
    await db.connect();
    log("DB Connected.");

    try {
        // PREPARE: Users and Tenants
        // Get Super Admin
        const superadminRes = await request('POST', '/auth/login', { email: 'superadmin@inmobiliaria.com', password: 'password123' });
        authHeadersSuperAdmin = { 'Authorization': `Bearer ${superadminRes.json.token}` };

        // Ensure we have two tenants
        const tenants = await queryDB(db, "SELECT id, name FROM tenants LIMIT 2");
        if (tenants.length < 2) {
            log("No hay 2 tenants, el test fallará o hay que crearlos.");
            return;
        }
        
        let tenantA = tenants[0].id;
        let tenantB = tenants[1].id;

        // Ensure users for tenants
        let adminA = await queryDB(db, "SELECT email FROM users WHERE tenant_id = $1 LIMIT 1", [tenantA]);
        let adminB = await queryDB(db, "SELECT email FROM users WHERE tenant_id = $1 LIMIT 1", [tenantB]);

        if(!adminA.length || !adminB.length) {
            log("No hay usuarios para testear tenant.");
            return;
        }

        const resA = await request('POST', '/auth/login', { email: adminA[0].email, password: 'password123' });
        const resB = await request('POST', '/auth/login', { email: adminB[0].email, password: 'password123' });

        authHeadersTenantA = { 'Authorization': `Bearer ${resA.json.token}` };
        authHeadersTenantB = { 'Authorization': `Bearer ${resB.json.token}` };

        // 1. CRUD CLIENTES
        log("\n--- 1. CRUD CLIENTES ---");
        const createClient = await request('POST', '/clients', { first_name: 'Auditoria', last_name: 'Cliente', phone: '123456789' }, authHeadersTenantA);
        const clientId = createClient.json.id;

        await request('GET', `/clients/${clientId}`, null, authHeadersTenantA);
        await request('PUT', `/clients/${clientId}`, { first_name: 'Auditoria Modificado' }, authHeadersTenantA);
        await request('DELETE', `/clients/${clientId}`, null, authHeadersTenantA);

        const checkDeleted = await queryDB(db, "SELECT id, deleted_at FROM clients WHERE id = $1", [clientId]);
        log(`Soft Delete check: deleted_at is ${checkDeleted[0]?.deleted_at !== null ? 'NOT NULL (OK)' : 'NULL (FAIL)'}`);

        // 2. CRUD LEADS & 3. KANBAN
        log("\n--- 2. CRUD LEADS & 3. KANBAN ---");
        const createLeadClient = await request('POST', '/clients', { first_name: 'Lead', last_name: 'Test', phone: '987654321' }, authHeadersTenantA);
        const leadClientId = createLeadClient.json.id;
        
        const createLead = await request('POST', '/leads', { client_id: leadClientId, status: 'NUEVO', source: 'Web' }, authHeadersTenantA);
        const leadId = createLead.json.id;

        await request('PUT', `/leads/${leadId}`, { status: 'CONTACTADO' }, authHeadersTenantA);
        await request('PUT', `/leads/${leadId}`, { status: 'RESERVA' }, authHeadersTenantA);
        const checkStatus = await queryDB(db, "SELECT status FROM leads WHERE id = $1", [leadId]);
        log(`Kanban status check: ${checkStatus[0]?.status}`);

        // Conversion
        await request('POST', `/leads/${leadId}/convert`, null, authHeadersTenantA);
        
        // 4. MULTI-TENANT
        log("\n--- 4. MULTI-TENANT ---");
        const tenantAClient = await request('POST', '/clients', { first_name: 'Juan Perez', last_name: 'A', phone: '111' }, authHeadersTenantA);
        const tenantBClient = await request('POST', '/clients', { first_name: 'Maria Gomez', last_name: 'B', phone: '222' }, authHeadersTenantB);
        
        const queryBwithA = await request('GET', `/clients/${tenantBClient.json.id}`, null, authHeadersTenantA);
        log(`Fuga de datos (Tenant A consulta Cliente B): HTTP ${queryBwithA.status} (Esperado 403 o 404)`);

        // 5. DASHBOARD
        log("\n--- 5. DASHBOARD ---");
        await request('GET', '/dashboard/stats', null, authHeadersTenantA);

        // 6 & 7. WHATSAPP
        log("\n--- 6. WHATSAPP SALIENTE ---");
        await request('POST', '/whatsapp/send', { phone: '12345', message: 'Test mensaje' }, authHeadersTenantA);

        log("\n--- 7. WHATSAPP ENTRANTE (WEBHOOK) ---");
        const payload = {
            event: "messages.upsert",
            data: {
                messages: [{
                    key: { remoteJid: "5491100000000@s.whatsapp.net", fromMe: false },
                    message: { conversation: "Hola, busco una casa" }
                }]
            }
        };
        await request('POST', `/whatsapp/webhook/${tenantA}`, payload);
        const checkWA = await queryDB(db, "SELECT id, status FROM leads WHERE source = 'WhatsApp Automático' ORDER BY created_at DESC LIMIT 1");
        
        // 8. RECORDATORIOS
        log("\n--- 8. RECORDATORIOS ---");
        await request('POST', '/whatsapp/reminders/run');

        // 9. AUDITORIA
        log("\n--- 9. AUDITORIA ---");
        const audits = await queryDB(db, "SELECT user_id, action, entity_type, entity_id FROM audit_logs ORDER BY created_at DESC LIMIT 5");

        // 10. RBAC
        log("\n--- 10. RBAC ---");
        const adminDelete = await request('DELETE', `/clients/${tenantAClient.json.id}`, null, authHeadersTenantA);
        log(`Admin eliminando cliente: ${adminDelete.status}`);

        // 11. EXPLAIN ANALYZE
        log("\n--- 11. PERFORMANCE ---");
        const explain = await queryDB(db, "EXPLAIN ANALYZE SELECT * FROM clients WHERE tenant_id = $1 AND deleted_at IS NULL", [tenantA]);
        log(JSON.stringify(explain));

    } catch (e) {
        log(`[FATAL ERROR] ${e}`);
    } finally {
        await db.end();
        fs.writeFileSync('evidencia_log.txt', logOutput);
        log("Log saved to evidencia_log.txt");
    }
}

runAudit();
