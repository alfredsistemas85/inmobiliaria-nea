const API_URL = 'http://127.0.0.1:3000/api';

async function runTest() {
    let port = 3000;

    console.log("1. Login");
    const loginRes = await fetch(`http://127.0.0.1:${port}/api/auth/login`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ email: 'superadmin@inmobiliaria.com', password: 'password123' })
    });
    
    if (!loginRes.ok) {
        const text = await loginRes.text();
        console.error("Login failed:", text);
        return;
    }
    const loginData = await loginRes.json();
    const token = loginData.access_token;
    console.log("Token obtained");

    console.log("1.5 Fetching Tenants to impersonate");
    const adminHeaders = { 'Authorization': `Bearer ${token}`, 'Content-Type': 'application/json' };
    const tenantRes = await fetch(`http://127.0.0.1:${port}/api/superadmin/tenants`, { headers: adminHeaders });
    const tenantData = await tenantRes.json();
    const tenantId = tenantData.data[0].id;
    console.log("Impersonating Tenant ID:", tenantId);

    console.log("2. Fetching Property and Client");
    const headers = { 'Authorization': `Bearer ${token}`, 'Content-Type': 'application/json', 'x-tenant-id': tenantId };
    
    const propRes = await fetch(`http://127.0.0.1:${port}/api/properties`, { headers });
    const propText = await propRes.text();
    let propData;
    try {
        propData = JSON.parse(propText);
    } catch(e) {
        console.error("Properties parse error:", e, propText);
        return;
    }
    const propertyId = propData.data[0].id;
    console.log("Property ID:", propertyId);

    const cliRes = await fetch(`http://127.0.0.1:${port}/api/clients`, { headers });
    const cliText = await cliRes.text();
    let cliData;
    try {
        cliData = JSON.parse(cliText);
    } catch(e) {
        console.error("Clients parse error:", e, cliText);
        return;
    }
    const clientId = cliData.data[0].id;
    const tenantClientId = cliData.data[1].id;
    console.log("Client IDs:", clientId, tenantClientId);

    console.log("3. Creating Contract");
    const payload = {
        property_id: propertyId,
        start_date: "2026-07-01",
        end_date: "2028-06-30",
        original_rent_amount: 500000,
        adjustment_method: "IPC",
        adjustment_frequency: "QUARTERLY",
        automation_mode: "SEMIAUTOMATIC",
        fixed_percentage: null,
        notification_days_before: 30,
        c_type: "HOUSING",
        c_destination: "HABITATIONAL",
        currency: "ARS",
        deposit_amount: 500000,
        commission_amount: 0,
        fees_amount: 0,
        status: "DRAFT",
        participants: [
            {
                client_id: clientId,
                p_role: "LANDLORD",
                percentage: 100,
                is_main: true,
                guarantees: []
            },
            {
                client_id: tenantClientId,
                p_role: "TENANT",
                percentage: 100,
                is_main: true,
                guarantees: []
            }
        ],
        terms: {
            allows_pets: false,
            allows_sublease: false,
            requires_inventory: false,
            requires_insurance: false,
            automatic_renewal: false,
            notice_days: 30,
            early_termination_penalty: "",
            observations: ""
        },
        clauses: [],
        template_id: null
    };

    const createRes = await fetch(`http://127.0.0.1:${port}/api/contracts/v2`, {
        method: 'POST',
        headers,
        body: JSON.stringify(payload)
    });
    
    const createText = await createRes.text();
    console.log("Create Status:", createRes.status);
    console.log("Create Response:", createText);
    
    if (createRes.status !== 201 && createRes.status !== 200) {
        console.error("Failed to create contract.");
        return;
    }
    
    const contractId = JSON.parse(createText).id || JSON.parse(createText).data?.id;

    console.log("4. Querying Contract", contractId);
    if (contractId) {
        const getRes = await fetch(`http://127.0.0.1:${port}/api/contracts/v2/${contractId}`, { headers });
        const getData = await getRes.json();
        console.log("Queried Contract:", JSON.stringify(getData, null, 2));
    }
}

runTest();
