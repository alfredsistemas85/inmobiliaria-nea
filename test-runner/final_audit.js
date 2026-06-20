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
    if (body && !path.includes('webhook')) log(`[HTTP Body] ${JSON.stringify(body)}`);
    try {
        const response = await fetch(`${API_URL}${path}`, {
            method,
            headers: { 'Content-Type': 'application/json', ...headers },
            body: body ? JSON.stringify(body) : undefined
        });
        const status = response.status;
        const text = await response.text();
        let json;
        try { json = JSON.parse(text); } catch (e) {}
        return { status, json: json || text };
    } catch (e) {
        return { status: 500, json: { error: e.message } };
    }
}

async function runFinalAudit() {
    log("========================================");
    log("AUDITORÍA FINAL FASE 3");
    log("========================================");
    
    const db = new Client({ connectionString: process.env.DATABASE_URL });
    await db.connect();
    
    try {
        log("\n--- PREPARANDO ENTORNO ---");
        // Ensure tenants exist
        let tenants = await db.query("SELECT id FROM tenants LIMIT 2");
        if (tenants.rows.length < 2) {
            await db.query("INSERT INTO tenants (cuit, dni_responsable, first_name, last_name, business_name) VALUES ('999', '999', 'Cross', 'Tenant', 'B') RETURNING id");
            tenants = await db.query("SELECT id FROM tenants LIMIT 2");
        }
        const tenantA = tenants.rows[0].id;
        const tenantB = tenants.rows[1].id;

        // Generate JWTs
        const secret = process.env.JWT_SECRET || 'secret_para_jwt';
        const tokenA = jwt.sign({ sub: '00000000-0000-0000-0000-000000000000', tenant_id: tenantA, role: 'tenant_agent', token_type: 'access', exp: 9999999999 }, secret);
        const tokenB = jwt.sign({ sub: '11111111-1111-1111-1111-111111111111', tenant_id: tenantB, role: 'tenant_agent', token_type: 'access', exp: 9999999999 }, secret);
        
        const authA = { 'Authorization': `Bearer ${tokenA}` };
        const authB = { 'Authorization': `Bearer ${tokenB}` };

        // 1. Prueba de duplicación de webhooks
        log("\n1. PRUEBA DE DUPLICACIÓN DE WEBHOOKS");
        const dupId = `MSG-DUP-${Date.now()}`;
        const phone = `54911${Math.floor(Math.random()*10000000)}`;
        const payload = {
            event: "messages.upsert",
            data: { messages: [{ key: { remoteJid: `${phone}@s.whatsapp.net`, fromMe: false, id: dupId }, message: { conversation: "Test duplicado" } }] }
        };
        await request('POST', `/whatsapp/webhook/${tenantA}`, payload);
        await new Promise(r => setTimeout(r, 1000));
        await request('POST', `/whatsapp/webhook/${tenantA}`, payload); // Segunda vez
        await new Promise(r => setTimeout(r, 1000));
        
        const dupCount = await db.query("SELECT COUNT(*) FROM messages WHERE external_id = $1", [dupId]);
        log(`Cantidad de mensajes insertados con id ${dupId}: ${dupCount.rows[0].count} (Esperado: 1)`);

        // 2. Prueba cross-tenant sobre conversaciones
        log("\n2. PRUEBA CROSS-TENANT SOBRE CONVERSACIONES");
        const convsA = await request('GET', '/whatsapp/conversations', null, authA);
        const targetConv = convsA.json.data.find(c => c.client_phone === phone);
        if (targetConv) {
            const crossRes = await request('GET', `/whatsapp/conversations/${targetConv.id}/messages`, null, authB);
            log(`Intento B lee A: Status ${crossRes.status}. Data length: ${crossRes.json.data?.length || 0} (Esperado: 0 o 403)`);
        }

        // 4. Prueba Evolution API caída
        log("\n4. PRUEBA EVOLUTION API CAÍDA");
        const evoFailRes = await request('POST', `/whatsapp/conversations/${targetConv.id}/messages`, { content: "Fallo red" }, authA);
        log(`Envío API Caída Status: ${evoFailRes.status} (Esperado: 500)`);
        const evoCheckMsg = await db.query("SELECT content FROM messages WHERE conversation_id = $1 AND content = 'Fallo red'", [targetConv.id]);
        log(`Mensajes insertados tras fallo: ${evoCheckMsg.rowCount} (Esperado: 0)`);

        // 5. Validación estados
        log("\n5. VALIDACIÓN ESTADOS");
        const statusCheck = await db.query("SELECT DISTINCT status, direction FROM messages WHERE conversation_id = $1", [targetConv.id]);
        log(`Estados en BBDD: ${JSON.stringify(statusCheck.rows)} (Esperado: delivered para inbound)`);

        // 6. Verificación assigned_user_id
        log("\n6. VERIFICACIÓN ASSIGNED_USER_ID");
        const assignedCheck = await db.query("SELECT assigned_user_id FROM conversations WHERE id = $1", [targetConv.id]);
        log(`assigned_user_id: ${assignedCheck.rows[0].assigned_user_id} (Columna existe)`);

        // 3. Prueba con 5.000 mensajes
        log("\n3. PRUEBA CON 5.000 MENSAJES (Rendimiento)");
        log(`Insertando 5,000 mensajes en conversación ${targetConv.id}...`);
        
        // Batch insert
        let values = [];
        for(let i=0; i<5000; i++) {
            values.push(`('${tenantA}', '${targetConv.id}', 'client', 'Msg ${i}', 'delivered', 'inbound')`);
        }
        await db.query(`INSERT INTO messages (tenant_id, conversation_id, sender_type, content, status, direction) VALUES ${values.join(',')}`);
        log("✅ 5,000 mensajes insertados.");
        
        // Fetch paginado
        const timeStart = Date.now();
        const page1 = await request('GET', `/whatsapp/conversations/${targetConv.id}/messages?page=10&limit=50`, null, authA);
        const timeDiff = Date.now() - timeStart;
        log(`Tiempo en recuperar página 10 con 5000 mensajes: ${timeDiff}ms`);
        log(`Total count reportado: ${page1.json.total}`);

        // 8. EXPLAIN ANALYZE
        log("\n8. EXPLAIN ANALYZE");
        const explainConvs = await db.query("EXPLAIN ANALYZE SELECT * FROM conversations WHERE tenant_id = $1 ORDER BY last_message_at DESC NULLS LAST LIMIT 50", [tenantA]);
        log("Conversations Query Plan:\n" + explainConvs.rows.map(r => r["QUERY PLAN"]).join('\n'));

        const explainMsgs = await db.query("EXPLAIN ANALYZE SELECT * FROM messages WHERE tenant_id = $1 AND conversation_id = $2 ORDER BY created_at ASC LIMIT 50", [tenantA, targetConv.id]);
        log("\nMessages Query Plan:\n" + explainMsgs.rows.map(r => r["QUERY PLAN"]).join('\n'));

        // 9. Verificación de Índices
        log("\n9. VERIFICACIÓN DE ÍNDICES");
        const indexes = await db.query(`
            SELECT indexname, indexdef 
            FROM pg_indexes 
            WHERE tablename IN ('conversations', 'messages') 
            AND indexname LIKE '%idx%'
        `);
        log(indexes.rows.map(r => `${r.indexname}: ${r.indexdef}`).join('\n'));

    } catch (e) {
        log(`[FATAL ERROR] ${e}`);
    } finally {
        await db.end();
        fs.writeFileSync('auditoria_final_fase3.txt', logOutput);
        log("\nLog completo guardado en auditoria_final_fase3.txt");
    }
}

runFinalAudit();
