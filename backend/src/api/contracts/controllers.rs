use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Html},
    Json, Extension,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;
use crate::core::security::jwt::Claims;
use rust_decimal::Decimal;

use super::models::Contract;
use super::dto::CreateContractDto;

pub async fn list_contracts(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<Contract>>, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::BAD_REQUEST)?;

    let contracts = sqlx::query_as::<_, Contract>(
        r#"SELECT id, tenant_id, property_id, start_date, end_date, 
           original_rent_amount, current_rent_amount, adjustment_method, adjustment_frequency,
           automation_mode, fixed_percentage, first_notification_days, second_notification_days, third_notification_days, requires_manual_approval, next_adjustment_date, last_adjustment_date, status
           FROM contracts WHERE tenant_id = $1"#
    )
    .bind(tenant_id)
    .fetch_all(&*pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(contracts))
}

pub async fn create_contract(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateContractDto>,
) -> Result<Json<Contract>, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::BAD_REQUEST)?;

    let contract = sqlx::query_as::<_, Contract>(
        r#"
        INSERT INTO contracts (tenant_id, property_id, start_date, end_date, original_rent_amount, current_rent_amount, adjustment_method, adjustment_frequency, automation_mode, fixed_percentage, first_notification_days)
        VALUES ($1, $2, $3, $4, $5, $5, $6, $7, $8, $9, $10)
        RETURNING id, tenant_id, property_id, start_date, end_date, original_rent_amount, current_rent_amount, adjustment_method, adjustment_frequency, automation_mode, fixed_percentage, first_notification_days, second_notification_days, third_notification_days, requires_manual_approval, next_adjustment_date, last_adjustment_date, status
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
    .fetch_one(&*pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(contract))
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

    let contract = sqlx::query_as::<_, ContractPdfData>(
        "SELECT c.start_date, c.end_date, c.current_rent_amount, c.adjustment_method, p.title as property_title FROM contracts c JOIN properties p ON c.property_id = p.id WHERE c.id = $1 AND c.tenant_id = $2"
    )
    .bind(id)
    .bind(tenant_id)
    .fetch_one(&*pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    let html = format!(
        r#"
        <html>
            <head><title>Contrato de Alquiler</title></head>
            <body style="font-family: Arial, sans-serif; padding: 40px; line-height: 1.6;">
                <h1 style="text-align: center;">CONTRATO DE LOCACIÓN</h1>
                <p>En la ciudad de ..., a los ... días del mes de ..., se celebra el presente contrato de locación entre <strong>[PROPIETARIO]</strong> y <strong>[INQUILINO]</strong>.</p>
                <h3>1. OBJETO</h3>
                <p>El locador cede en locación el inmueble sito en <strong>{}</strong>.</p>
                <h3>2. PRECIO Y PLAZO</h3>
                <p>El plazo de la locación es desde el {} hasta el {}. El canon locativo se fija en la suma de <strong>${}</strong> mensuales.</p>
                <h3>3. AJUSTE</h3>
                <p>El alquiler se actualizará bajo el método <strong>{:?}</strong>.</p>
                <br><br><br>
                <div style="display: flex; justify-content: space-around;">
                    <div><hr>Firma Locador</div>
                    <div><hr>Firma Locatario</div>
                </div>
            </body>
        </html>
        "#,
        contract.property_title,
        contract.start_date,
        contract.end_date,
        contract.current_rent_amount.unwrap_or_default(),
        contract.adjustment_method
    );

    Ok(Html(html))
}
