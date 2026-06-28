use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use super::models::ElectronicInvoice;

pub struct AfipRepository;

impl AfipRepository {
    pub async fn insert_invoice(
        tx: &mut Transaction<'_, Postgres>,
        invoice: &ElectronicInvoice,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO electronic_invoices (
                id, tenant_id, receipt_id, invoice_type, point_of_sale, invoice_number, 
                cae, cae_expiration, status, request_payload, response_payload, 
                pdf_path, xml_path, created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            "#
        )
        .bind(invoice.id)
        .bind(invoice.tenant_id)
        .bind(invoice.receipt_id)
        .bind(&invoice.invoice_type)
        .bind(invoice.point_of_sale)
        .bind(invoice.invoice_number)
        .bind(&invoice.cae)
        .bind(invoice.cae_expiration)
        .bind(&invoice.status)
        .bind(&invoice.request_payload)
        .bind(&invoice.response_payload)
        .bind(&invoice.pdf_path)
        .bind(&invoice.xml_path)
        .bind(invoice.created_at)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn get_last_invoice_number(
        pool: &PgPool,
        tenant_id: Uuid,
        point_of_sale: i32,
        invoice_type: &str,
    ) -> Result<i32, sqlx::Error> {
        // En un caso real, esto puede venir directamente de AFIP o de la tabla.
        // Simularemos consultando la tabla local.
        let result = sqlx::query_scalar::<_, i32>(
            r#"
            SELECT COALESCE(MAX(invoice_number), 0)
            FROM electronic_invoices
            WHERE tenant_id = $1 AND point_of_sale = $2 AND invoice_type = $3
            "#
        )
        .bind(tenant_id)
        .bind(point_of_sale)
        .bind(invoice_type)
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn update_invoice_approval(
        pool: &PgPool,
        invoice_id: Uuid,
        cae: &str,
        cae_expiration: chrono::NaiveDate,
        response_payload: serde_json::Value,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE electronic_invoices
            SET status = 'APPROVED',
                cae = $1,
                cae_expiration = $2,
                response_payload = $3
            WHERE id = $4
            "#
        )
        .bind(cae)
        .bind(cae_expiration)
        .bind(&response_payload)
        .bind(invoice_id)
        .execute(pool)
        .await?;

        Ok(())
    }
}
