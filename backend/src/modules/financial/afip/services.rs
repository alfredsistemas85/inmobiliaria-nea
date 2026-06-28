use chrono::{Duration, Utc};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use super::{
    models::ElectronicInvoice,
    repositories::AfipRepository,
};

pub struct AfipService {
    pool: PgPool,
}

impl AfipService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Emits an electronic invoice by contacting AFIP (or DummyProvider for testing).
    pub async fn emit_invoice(
        &self,
        tenant_id: Uuid,
        receipt_id: Uuid,
        invoice_type: &str, // e.g. "Factura C"
        point_of_sale: i32,
    ) -> Result<ElectronicInvoice, String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        // 1. Determine next invoice number
        let next_number = AfipRepository::get_last_invoice_number(&self.pool, tenant_id, point_of_sale, invoice_type)
            .await
            .map_err(|e| e.to_string())? + 1;

        // 2. Insert as PENDING
        let request_payload = json!({
            "PtoVta": point_of_sale,
            "CbteTipo": invoice_type,
            "Concepto": 1,
            "DocTipo": 80, // CUIT
            // En un entorno real se mandarían montos, fechas, etc.
        });

        let mut invoice = ElectronicInvoice {
            id: Uuid::new_v4(),
            tenant_id,
            receipt_id,
            invoice_type: invoice_type.to_string(),
            point_of_sale,
            invoice_number: next_number,
            cae: None,
            cae_expiration: None,
            status: "PENDING".to_string(),
            request_payload: Some(request_payload),
            response_payload: None,
            pdf_path: None,
            xml_path: None,
            created_at: Utc::now(),
        };

        AfipRepository::insert_invoice(&mut tx, &invoice).await.map_err(|e| e.to_string())?;
        tx.commit().await.map_err(|e| e.to_string())?;

        // 3. Call "DummyProvider" (Mock AFIP API)
        let cae_simulado = format!("73{}{}", point_of_sale, next_number);
        let vencimiento_simulado = (Utc::now() + Duration::days(10)).naive_utc().date();
        let response_payload = json!({
            "CAE": cae_simulado,
            "CAEFchVto": vencimiento_simulado.format("%Y%m%d").to_string(),
            "Resultado": "A"
        });

        // 4. Update approval
        AfipRepository::update_invoice_approval(
            &self.pool,
            invoice.id,
            &cae_simulado,
            vencimiento_simulado,
            response_payload.clone(),
        )
        .await
        .map_err(|e| e.to_string())?;

        invoice.status = "APPROVED".to_string();
        invoice.cae = Some(cae_simulado);
        invoice.cae_expiration = Some(vencimiento_simulado);
        invoice.response_payload = Some(response_payload);

        Ok(invoice)
    }
}
