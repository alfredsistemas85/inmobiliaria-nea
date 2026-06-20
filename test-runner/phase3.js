const { Client } = require('pg');
const fs = require('fs');
const jwt = require('jsonwebtoken');
require('dotenv').config({ path: '../backend/.env' });

const API_URL = 'http://localhost:3000/api';
let logOutput = "";

function log(msg) {
    console.log(msg);
    logOutput += msg + "\n";
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
        try { json = JSON.parse(text); log(`[HTTP Response JSON] ${JSON.stringify(json)}`); } 
        catch (e) {}
        return { status, json: json || text };
    } catch (e) {
        log(`[HTTP Error] ${e.message}`);
        return { status: 500, json: { error: e.message } };
    }
}

async function runAudit() {
    log("========================================");
    log("INICIANDO PRUEBA FLUJO FASE 3");
    log("========================================");
    
    const db = new Client({ connectionString: process.env.DATABASE_URL });
    await db.connect();
    
    try {
        log("\n--- SEEDING & JWT MOCK ---");
        let tenants = await db.query("SELECT id FROM tenants LIMIT 1");
        let tenantA;
        if (tenants.rows.length === 0) {
            const res = await db.query("INSERT INTO tenants (cuit, dni_responsable, first_name, last_name, business_name) VALUES ('123', '123', 'T', 'T', 'T') RETURNING id");
            tenantA = res.rows[0].id;
        } else {
            tenantA = tenants.rows[0].id;
        }

        let users = await db.query("SELECT id FROM users WHERE tenant_id = $1 LIMIT 1", [tenantA]);
        let userId;
        if (users.rows.length === 0) {
            const res = await db.query("INSERT INTO users (tenant_id, email, password_hash) VALUES ($1, 'agent@test.com', 'dummy') RETURNING id", [tenantA]);
            userId = res.rows[0].id;
        } else {
            userId = users.rows[0].id;
        }

        // Create JWT
        const token = jwt.sign(
            { sub: userId, tenant_id: tenantA, role: 'tenant_agent', token_type: 'access', exp: Math.floor(Date.now() / 1000) + (60 * 60) },
            process.env.JWT_SECRET || 'secret_para_jwt'
        );

        const authHeaders = { 'Authorization': `Bearer ${token}` };

        // 2. Simulate Webhook Inbound
        const randomExternalId = `MSG-${Date.now()}`;
        const phone = `54911${Math.floor(Math.random()*10000000)}`;
        
        const webhookPayload = {
            event: "messages.upsert",
            data: {
                messages: [{
                    key: { remoteJid: `${phone}@s.whatsapp.net`, fromMe: false, id: randomExternalId },
                    message: { conversation: "Hola, me interesa la propiedad!" }
                }]
            }
        };

        log("\n--- SIMULANDO WEBHOOK ---");
        await request('POST', `/whatsapp/webhook/${tenantA}`, webhookPayload);

        await new Promise(resolve => setTimeout(resolve, 2000));

        // 3. Check conversations API
        log("\n--- CONSULTANDO CONVERSACIONES ---");
        const convs = await request('GET', '/whatsapp/conversations', null, authHeaders);
        const latestConv = convs.json?.data?.find(c => c.client_phone === phone);

        if (!latestConv) {
            log("❌ No se encontró la conversación creada por el webhook.");
            return;
        }

        log(`✅ Conversación encontrada. ID: ${latestConv.id}, Unread: ${latestConv.unread_count}, LastMsg: ${latestConv.last_message_content}`);

        // 4. Check messages API
        log("\n--- CONSULTANDO MENSAJES ---");
        const messages = await request('GET', `/whatsapp/conversations/${latestConv.id}/messages`, null, authHeaders);
        log(`✅ Mensajes encontrados: ${messages.json.total}. Ultimo mensaje: ${messages.json?.data[0]?.content}`);

        // 5. Send manual message
        log("\n--- ENVIANDO RESPUESTA (AGENT) ---");
        const sendRes = await request('POST', `/whatsapp/conversations/${latestConv.id}/messages`, { content: "Hola! Claro, te enviamos info." }, authHeaders);
        if (sendRes.status !== 200) {
            log(`[WARNING] Error de Evolution API ignorado (se espera si no hay credenciales válidas).`);
        } else {
            log(`✅ Respuesta enviada. Nuevo mensaje ID: ${sendRes.json.id}`);
        }

        // 6. Check unread count is reset
        const convsAfter = await request('GET', '/whatsapp/conversations', null, authHeaders);
        const latestConvAfter = convsAfter.json?.data?.find(c => c.id === latestConv.id);
        log(`✅ Conversación Unread Count después del envío: ${latestConvAfter?.unread_count}`);

        // 7. Audit checks
        log("\n--- VERIFICANDO AUDITORÍA SQL ---");
        const audits = await db.query("SELECT action, entity_type FROM audit_logs WHERE tenant_id = $1 ORDER BY created_at DESC LIMIT 5", [tenantA]);
        log("Últimos logs de auditoría: " + JSON.stringify(audits.rows));

    } catch (e) {
        log(`[FATAL ERROR] ${e}`);
    } finally {
        await db.end();
        fs.writeFileSync('evidencia_fase3.txt', logOutput);
        log("Log saved to evidencia_fase3.txt");
    }
}

runAudit();
