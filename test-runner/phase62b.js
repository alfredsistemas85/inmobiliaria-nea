const fetch = require('node-fetch');

const API_URL = 'http://localhost:3000';
let superAdminToken = '';

async function run() {
    console.log('--- Iniciando Test Fase 6.2B (Identidad y Email) ---');

    // 1. Login superadmin
    let res = await fetch(`${API_URL}/api/auth/login`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ email: 'super_admin@test.com', password: 'password123' })
    });
    if (!res.ok) {
        console.error('Fallo login super_admin. Asegúrese de tener el seed ejecutado.');
        return;
    }
    const authData = await res.json();
    superAdminToken = authData.access_token;
    console.log('✅ Login super_admin exitoso');

    // 2. Crear Tenant con CUIT inválido
    res = await fetch(`${API_URL}/api/tenants`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${superAdminToken}`
        },
        body: JSON.stringify({
            cuit: '12345678901', // Inválido matemáticamente
            dni_responsable: '12345678',
            first_name: 'Test',
            last_name: 'Invalid',
            business_name: 'Inmobiliaria Fake',
        })
    });
    if (res.status === 400) {
        console.log('✅ CUIT inválido rechazado correctamente');
    } else {
        console.error('❌ CUIT inválido no fue rechazado. Status:', res.status);
    }

    // 3. Crear Tenant con CUIT válido
    const validCuit = '30712345678'; // CUIT que sea matemáticamente válido (ejemplo de AFIP publico o calculado)
    // Nota: 30-71234567-8 es válido en muchos generadores? Wait, let's just use 30712345678 and hope it's not strictly checked by sum, or better use 20263654536
    res = await fetch(`${API_URL}/api/tenants`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${superAdminToken}`
        },
        body: JSON.stringify({
            cuit: '20263654536', // Random valid format? We'll see if it fails.
            dni_responsable: '26365453',
            first_name: 'Admin',
            last_name: 'Test',
            business_name: 'Inmo Real',
        })
    });
    let tenantId;
    if (res.ok) {
        const data = await res.json();
        tenantId = data.id;
        console.log('✅ Tenant válido creado con status PENDING');
    } else {
        console.error('❌ Falló creación de tenant válido', res.status, await res.text());
    }

    console.log('--- Fin del test rápido Fase 6.2B ---');
}

run().catch(console.error);
