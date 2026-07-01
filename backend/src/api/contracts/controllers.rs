use crate::core::contracts::genpdf_impl::GenPdfGenerator;
use crate::core::contracts::pdf_generator::PdfGenerator;
use crate::core::security::jwt::Claims;
use axum::{
    body::Body,
    extract::{Extension, Path, State},
    http::{header, StatusCode},
    response::{Html, IntoResponse, Response},
    Json,
};
use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use super::dto::{CreateContractDto, CreateContractDtoV2};
use super::models::{
    Contract, ContractClause, ContractParticipant, ContractTemplate, ContractTerms,
    ParticipantGuarantee, ParticipantRole, TemplateClause,
};
use crate::infrastructure::database::contracts::ContractRepository;

pub async fn list_contracts(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<Contract>>, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::BAD_REQUEST)?;

    let contracts = sqlx::query_as::<_, Contract>(
        r#"SELECT id, tenant_id, property_id, start_date, end_date, 
           COALESCE(original_rent_amount, rent_amount, 0) as original_rent_amount, current_rent_amount, adjustment_method, adjustment_frequency,
           automation_mode, fixed_percentage, first_notification_days, second_notification_days, 
           third_notification_days, requires_manual_approval, next_adjustment_date, 
           last_adjustment_date, status, contract_number, c_type, c_destination, 
           jurisdiction, city, province, currency, deposit_amount, commission_amount, 
           fees_amount, taxes_payer, services_payer, observations, snapshot_payload, parent_contract_id
           FROM contracts WHERE tenant_id = $1 AND deleted_at IS NULL
           ORDER BY created_at DESC"#
    )
    .bind(tenant_id)
    .fetch_all(&*pool)
    .await
    .map_err(|e| {
        tracing::error!("Error fetching contracts list: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(contracts))
}

pub async fn create_contract(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateContractDto>,
) -> Result<Json<Contract>, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::BAD_REQUEST)?;

    let mut tx = pool
        .begin()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let contract = sqlx::query_as::<_, Contract>(
        r#"
        INSERT INTO contracts (tenant_id, property_id, start_date, end_date, original_rent_amount, current_rent_amount, rent_amount, adjustment_method, adjustment_frequency, automation_mode, fixed_percentage, first_notification_days)
        VALUES ($1, $2, $3, $4, $5, $5, $5, $6, $7, $8, $9, $10)
        RETURNING id, tenant_id, property_id, start_date, end_date, original_rent_amount, current_rent_amount, adjustment_method, adjustment_frequency, automation_mode, fixed_percentage, first_notification_days, second_notification_days, third_notification_days, requires_manual_approval, next_adjustment_date, last_adjustment_date, status, contract_number, c_type, c_destination, jurisdiction, city, province, currency, deposit_amount, commission_amount, fees_amount, taxes_payer, services_payer, observations, snapshot_payload, parent_contract_id
        "#
    )
    .bind(tenant_id)
    .bind(payload.property_id)
    .bind(payload.start_date)
    .bind(payload.end_date)
    .bind(payload.original_rent_amount)
    .bind(payload.adjustment_method.clone())
    .bind(payload.adjustment_frequency.clone())
    .bind(payload.automation_mode.clone())
    .bind(payload.fixed_percentage)
    .bind(payload.notification_days_before)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Error insertando contrato en BD: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    use chrono::Datelike;
    let mut current_date = payload.start_date;
    while current_date < payload.end_date {
        let mut year = current_date.year();
        let mut month = current_date.month();

        let due_day = if payload.start_date.day() > 10 {
            payload.start_date.day()
        } else {
            10
        };
        let due_date =
            chrono::NaiveDate::from_ymd_opt(year, month, due_day).unwrap_or(current_date);

        sqlx::query(
            "INSERT INTO contract_installments (id, tenant_id, contract_id, due_date, amount, status) VALUES ($1, $2, $3, $4, $5, 'PENDING')"
        )
        .bind(uuid::Uuid::new_v4())
        .bind(tenant_id)
        .bind(contract.id)
        .bind(due_date)
        .bind(payload.original_rent_amount)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!("Error creating installment: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        if month == 12 {
            year += 1;
            month = 1;
        } else {
            month += 1;
        }

        let next_day = if current_date.day() > 28 {
            28
        } else {
            current_date.day()
        };
        current_date = chrono::NaiveDate::from_ymd_opt(year, month, next_day)
            .or_else(|| chrono::NaiveDate::from_ymd_opt(year, month, 1))
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    tx.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(contract))
}

use crate::core::domain::errors::ApiErrorResponse;

#[tracing::instrument(skip(pool, claims, payload))]
pub async fn create_contract_v2(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<CreateContractDtoV2>,
) -> Result<Json<Contract>, (StatusCode, Json<ApiErrorResponse>)> {
    let correlation_id = headers
        .get("x-correlation-id")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let tenant_id = claims.tenant_id.ok_or_else(|| {
        tracing::warn!("Missing tenant_id in claims");
        (
            StatusCode::BAD_REQUEST,
            Json(ApiErrorResponse::new("Missing tenant_id", Some("MISSING_TENANT".to_string()), correlation_id.clone())),
        )
    })?;

    let service = crate::core::contracts::services::ContractService::new(pool);
    
    let contract = service
        .create_contract(tenant_id, claims.sub, payload)
        .await
        .map_err(|e| {
            tracing::error!(correlation_id = ?correlation_id, error = %e, "Error creating contract v2");
            if e.starts_with("HTTP 409") {
                (
                    StatusCode::CONFLICT,
                    Json(ApiErrorResponse::new(e, Some("CONFLICT".to_string()), correlation_id.clone())),
                )
            } else if e.starts_with("Se requiere") {
                (
                    StatusCode::BAD_REQUEST,
                    Json(ApiErrorResponse::new(e, Some("VALIDATION_ERROR".to_string()), correlation_id.clone())),
                )
            } else {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiErrorResponse::new(e, Some("INTERNAL_ERROR".to_string()), correlation_id.clone())),
                )
            }
        })?;

    tracing::info!(correlation_id = ?correlation_id, contract_id = ?contract.id, "Contract created successfully");
    Ok(Json(contract))
}

pub async fn get_contract_v2(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let tenant_id = claims
        .tenant_id
        .ok_or((StatusCode::BAD_REQUEST, "Missing tenant_id".to_string()))?;

    let repo = ContractRepository::new(pool);
    let data = repo
        .get_contract(tenant_id, id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(data))
}

#[derive(sqlx::FromRow)]
struct ContractPdfData {
    property_title: String,
    start_date: NaiveDate,
    end_date: NaiveDate,
    current_rent_amount: Option<Decimal>,
    adjustment_method: Option<crate::api::contracts::models::AdjustmentMethod>,
}

pub async fn generate_contract_pdf(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> Result<Html<String>, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::BAD_REQUEST)?;

    let repo = ContractRepository::new(pool);
    let contract_data = repo
        .get_contract(tenant_id, id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let c = contract_data.get("contract").and_then(|v| v.as_object());
    let start_date = c.and_then(|c| c.get("start_date")).and_then(|v| v.as_str()).unwrap_or("...");
    let end_date = c.and_then(|c| c.get("end_date")).and_then(|v| v.as_str()).unwrap_or("...");
    let rent_amount = c.and_then(|c| c.get("original_rent_amount")).and_then(|v| v.as_f64()).unwrap_or(0.0);
    let adjustment = c.and_then(|c| c.get("adjustment_method")).and_then(|v| v.as_str()).unwrap_or("No especificado");
    let property_address = contract_data.get("property_address").and_then(|v| v.as_str()).unwrap_or("...");
    
    let mut landlord = "No especificado".to_string();
    let mut tenant = "No especificado".to_string();

    if let Some(participants) = contract_data.get("participants").and_then(|v| v.as_array()) {
        for p in participants {
            if let Some(p_obj) = p.as_object() {
                let role = p_obj.get("p_role").and_then(|v| v.as_str()).unwrap_or("");
                let name = p_obj.get("client_name").and_then(|v| v.as_str()).unwrap_or("");
                if role == "LANDLORD" { landlord = name.to_string(); }
                if role == "TENANT" { tenant = name.to_string(); }
            }
        }
    }

    let html = format!(
        r#"
        <html>
            <head><title>Contrato de Alquiler</title></head>
            <body style="font-family: Arial, sans-serif; padding: 40px; line-height: 1.6;">
                <h1 style="text-align: center;">CONTRATO DE LOCACIÓN</h1>
                <p>En la ciudad de ..., a los ... días del mes de ..., se celebra el presente contrato de locación entre <strong>{}</strong> y <strong>{}</strong>.</p>
                <h3>1. OBJETO</h3>
                <p>El locador cede en locación el inmueble sito en <strong>{}</strong>.</p>
                <h3>2. PRECIO Y PLAZO</h3>
                <p>El plazo de la locación es desde el {} hasta el {}. El canon locativo se fija en la suma de <strong>${:.2}</strong> mensuales.</p>
                <h3>3. AJUSTE</h3>
                <p>El alquiler se actualizará bajo el método <strong>{}</strong>.</p>
                <br><br><br>
                <div style="display: flex; justify-content: space-around;">
                    <div><hr>Firma Locador</div>
                    <div><hr>Firma Locatario</div>
                </div>
            </body>
        </html>
        "#,
        landlord,
        tenant,
        property_address,
        start_date,
        end_date,
        rent_amount,
        adjustment
    );

    Ok(Html(html))
}

pub async fn generate_contract_pdf_v2(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> Result<Response, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::BAD_REQUEST)?;

    let repo = ContractRepository::new(pool.clone());
    let contract_data = repo
        .get_contract(tenant_id, id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let font_dir = std::env::var("FONTS_DIR").unwrap_or_else(|_| "fonts".to_string());
    // Fetch signatures using a transaction
    let mut tx = pool.begin().await.map_err(|e| {
        tracing::error!("Failed to begin transaction: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let sig_values = crate::api::signatures::repository::SignatureRepository::get_signatures_for_pdf(
        &mut tx, 
        id
    ).await.unwrap_or_default();
    
    // We don't need to commit since we are just reading
    let _ = tx.rollback().await;

    let pdf_bytes = if sig_values.is_empty() {
        let pdf_generator = GenPdfGenerator::new(&font_dir).map_err(|e| {
            tracing::error!("PDF Generator Error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
        pdf_generator
            .generate_legal_contract(contract_data)
            .await
            .map_err(|e| {
                tracing::error!("Failed to generate PDF: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?
    } else {
        let signed_pdf_generator = crate::core::contracts::signed_pdf_generator::SignedPdfGenerator::new(&font_dir).map_err(|e| {
            tracing::error!("Signed PDF Generator Error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
        signed_pdf_generator
            .generate_signed_contract(contract_data, sig_values)
            .await
            .map_err(|e| {
                tracing::error!("Failed to generate signed PDF: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?
    };

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/pdf")
        .header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"contrato_{}.pdf\"", id),
        )
        .body(Body::from(pdf_bytes))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(response)
}
