use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;
use tracing::{info, error, warn};

use super::pdf_generator::{CertificateData, PdfGenerator};
use super::genpdf_impl::GenPdfGenerator;
use crate::core::storage::StorageProvider;
use crate::core::storage::supabase::SupabaseStorageProvider;
use crate::infrastructure::evolution::client::EvolutionClient;
use chrono::NaiveDate;
use rust_decimal::Decimal;

#[derive(Clone)]
pub struct RentAdjustmentApproved {
    pub tenant_id: Uuid,
    pub contract_id: Uuid,
    pub adjustment_id: Uuid,
    pub user_id: Uuid,
}

pub async fn handle_rent_adjustment_approved(event: RentAdjustmentApproved, pool: Arc<PgPool>) {
    info!("Processing RentAdjustmentApproved event for adjustment {}", event.adjustment_id);

    // Fetch required details
    #[derive(sqlx::FromRow)]
    struct AdjRecord {
        new_amount: Option<rust_decimal::Decimal>,
        previous_amount: Option<rust_decimal::Decimal>,
        adjustment_method: String,
        percentage_applied: Option<rust_decimal::Decimal>,
        effective_date: chrono::NaiveDate,
        t_first: String,
        t_last: String,
        t_phone: Option<String>,
        o_first: String,
        o_last: String,
        o_phone: Option<String>,
        street: String,
        prop_number: String,
        city: String,
        real_estate_name: String,
        u_first: String,
        u_last: String,
        agent_phone: Option<String>,
    }

    let record = match sqlx::query_as::<_, AdjRecord>(
        r#"
        SELECT 
            ra.new_amount, ra.previous_amount, ra.adjustment_method, ra.percentage_applied, ra.effective_date,
            t.first_name as t_first, t.last_name as t_last, t.phone as t_phone,
            o.first_name as o_first, o.last_name as o_last, o.phone as o_phone,
            prop.street, prop.number as prop_number, prop.city,
            ten.name as real_estate_name,
            u.first_name as u_first, u.last_name as u_last, u.phone as agent_phone
        FROM rent_adjustments ra
        JOIN contracts c ON ra.contract_id = c.id
        JOIN properties prop ON c.property_id = prop.id
        JOIN clients t ON c.client_id = t.id
        JOIN clients o ON prop.owner_id = o.id
        JOIN tenants ten ON ra.tenant_id = ten.id
        JOIN users u ON ra.approved_by = u.id
        WHERE ra.id = $1
        "#
    ).bind(event.adjustment_id).fetch_one(&*pool).await {
        Ok(r) => r,
        Err(e) => {
            error!("Failed to fetch adjustment details: {}", e);
            return;
        }
    };

    let tenant_name = format!("{} {}", record.t_first, record.t_last);
    let owner_name = format!("{} {}", record.o_first, record.o_last);
    let approver_name = format!("{} {}", record.u_first, record.u_last);
    let address = format!("{} {}, {}", record.street, record.prop_number, record.city);

    let cert_data = CertificateData {
        tenant_id: event.tenant_id,
        contract_id: event.contract_id,
        rent_adjustment_id: event.adjustment_id,
        real_estate_name: record.real_estate_name,
        owner_name: owner_name.clone(),
        tenant_name: tenant_name.clone(),
        property_address: address,
        previous_amount: record.previous_amount.unwrap_or(Decimal::new(0, 0)),
        new_amount: record.new_amount.unwrap_or(Decimal::new(0, 0)),
        method: record.adjustment_method,
        percentage: record.percentage_applied.unwrap_or(Decimal::new(0, 0)),
        effective_date: record.effective_date,
        approver_name,
        issue_date: chrono::Utc::now().naive_utc().date(),
    };

    // 1. Generate PDF
    let font_dir = std::env::var("FONTS_DIR").unwrap_or_else(|_| "fonts".to_string());
    let pdf_gen_result = GenPdfGenerator::new(&font_dir);
    
    if let Err(e) = pdf_gen_result {
        error!("Failed to initialize PDF Generator: {}", e);
        return;
    }
    
    let pdf_generator = pdf_gen_result.unwrap();

    let pdf_bytes = match pdf_generator.generate_adjustment_certificate(cert_data).await {
        Ok(b) => b,
        Err(e) => {
            error!("Failed to generate PDF for adjustment {}: {}", event.adjustment_id, e);
            return;
        }
    };
    
    let _ = log_audit(&pool, event.tenant_id, event.user_id, event.contract_id, "PDF_GENERATED", serde_json::json!({"adjustment_id": event.adjustment_id})).await;

    // 2. Upload to Supabase Storage
    let storage_provider = SupabaseStorageProvider::new();
    let file_path = format!("{}/{}_adjustment.pdf", event.tenant_id, event.adjustment_id);
    
    let upload_result = storage_provider.upload_document(
        "certificados",
        &file_path,
        pdf_bytes,
        "application/pdf",
    ).await;

    if let Err(e) = upload_result {
        error!("Failed to upload PDF to storage: {}", e);
        return;
    }
    
    let _ = sqlx::query(
        "INSERT INTO documents (tenant_id, uploaded_by, entity_type, entity_id, file_name, file_size, mime_type, storage_path, document_type, rent_adjustment_id) 
         VALUES ($1, $2, 'contract', $3, $4, $5, 'application/pdf', $6, 'ADJUSTMENT_CERTIFICATE', $7)"
    )
    .bind(event.tenant_id)
    .bind(event.user_id)
    .bind(event.contract_id)
    .bind(format!("Ajuste_{}.pdf", record.effective_date))
    .bind(0)
    .bind(file_path.clone())
    .bind(event.adjustment_id)
    .execute(&*pool).await;

    let _ = log_audit(&pool, event.tenant_id, event.user_id, event.contract_id, "DOCUMENT_STORED", serde_json::json!({"path": file_path})).await;

    // 3. Generate Signed URL
    let signed_url = match storage_provider.generate_signed_url("certificados", &file_path, 604800).await {
        Ok(url) => url,
        Err(e) => {
            error!("Failed to generate signed URL: {}", e);
            return;
        }
    };
    
    let _ = log_audit(&pool, event.tenant_id, event.user_id, event.contract_id, "SIGNED_URL_CREATED", serde_json::json!({"expires_in": 604800})).await;

    // 4. Send WhatsApp Notifications
    let whatsapp = EvolutionClient::new();
    let file_name = format!("Actualizacion_Alquiler_{}.pdf", record.effective_date);
    
    let new_amount_str = record.new_amount.unwrap_or(Decimal::new(0, 0)).to_string();
    let tenant_msg = format!("Estimado {}, adjuntamos el certificado de actualización de su alquiler. Nuevo valor: ${}", tenant_name, new_amount_str);
    let owner_msg = format!("Estimado {}, se ha procesado y notificado el ajuste de alquiler a su inquilino. Nuevo valor: ${}", owner_name, new_amount_str);
    let agent_msg = format!("Actualización procesada exitosamente para el contrato {} (${})", event.contract_id, new_amount_str);

    send_whatsapp_with_audit(&whatsapp, &pool, record.t_phone, &tenant_msg, &signed_url, &file_name, event.tenant_id, event.user_id, event.contract_id).await;
    send_whatsapp_with_audit(&whatsapp, &pool, record.o_phone, &owner_msg, &signed_url, &file_name, event.tenant_id, event.user_id, event.contract_id).await;
    send_whatsapp_with_audit(&whatsapp, &pool, record.agent_phone, &agent_msg, &signed_url, &file_name, event.tenant_id, event.user_id, event.contract_id).await;
    
    info!("Finished processing RentAdjustmentApproved event for {}", event.adjustment_id);
}

async fn send_whatsapp_with_audit(
    client: &EvolutionClient,
    pool: &PgPool,
    phone: Option<String>,
    msg: &str,
    url: &str,
    file_name: &str,
    tenant_id: Uuid,
    user_id: Uuid,
    contract_id: Uuid,
) {
    if let Some(p) = phone {
        if !p.is_empty() {
            match client.send_media(&p, msg, url, file_name).await {
                Ok(_) => {
                    let _ = log_audit(pool, tenant_id, user_id, contract_id, "WHATSAPP_SENT", serde_json::json!({"phone": p})).await;
                }
                Err(e) => {
                    let _ = log_audit(pool, tenant_id, user_id, contract_id, "WHATSAPP_FAILED", serde_json::json!({"phone": p, "error": e})).await;
                }
            }
        }
    }
}

async fn log_audit(
    pool: &PgPool,
    tenant_id: Uuid,
    user_id: Uuid,
    contract_id: Uuid,
    action: &str,
    new_data: serde_json::Value,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO audit_logs (tenant_id, user_id, contract_id, action, old_data, new_data, method)
        VALUES ($1, $2, $3, $4, '{}', $5, 'SYSTEM')
        "#
    )
    .bind(tenant_id)
    .bind(user_id)
    .bind(contract_id)
    .bind(action)
    .bind(new_data)
    .execute(pool)
    .await?;
    Ok(())
}
