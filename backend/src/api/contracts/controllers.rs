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

use super::models::{Contract, ContractParticipant, ParticipantGuarantee, ParticipantRole};
use super::dto::{CreateContractDto, CreateContractDtoV2};

pub async fn list_contracts(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<Contract>>, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::BAD_REQUEST)?;

    let contracts = sqlx::query_as::<_, Contract>(
        r#"SELECT id, tenant_id, property_id, start_date, end_date, 
           original_rent_amount, current_rent_amount, adjustment_method, adjustment_frequency,
           automation_mode, fixed_percentage, first_notification_days, second_notification_days, third_notification_days, requires_manual_approval, next_adjustment_date, last_adjustment_date, status
           FROM contracts WHERE tenant_id = $1 AND deleted_at IS NULL
           ORDER BY created_at DESC"#
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

    let mut tx = pool.begin().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let contract = sqlx::query_as::<_, Contract>(
        r#"
        INSERT INTO contracts (tenant_id, property_id, start_date, end_date, original_rent_amount, current_rent_amount, rent_amount, adjustment_method, adjustment_frequency, automation_mode, fixed_percentage, first_notification_days)
        VALUES ($1, $2, $3, $4, $5, $5, $5, $6, $7, $8, $9, $10)
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
        
        let due_day = if payload.start_date.day() > 10 { payload.start_date.day() } else { 10 };
        let due_date = chrono::NaiveDate::from_ymd_opt(year, month, due_day).unwrap_or(current_date);
        
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
        
        let next_day = if current_date.day() > 28 { 28 } else { current_date.day() };
        current_date = chrono::NaiveDate::from_ymd_opt(year, month, next_day)
            .unwrap_or_else(|| chrono::NaiveDate::from_ymd_opt(year, month, 1).unwrap());
    }

    tx.commit().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(contract))
}

pub async fn create_contract_v2(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateContractDtoV2>,
) -> Result<Json<Contract>, (StatusCode, String)> {
    let tenant_id = claims.tenant_id.ok_or((StatusCode::BAD_REQUEST, "Missing tenant_id".to_string()))?;

    // Validations: At least 1 main landlord and 1 main tenant
    let has_main_landlord = payload.participants.iter().any(|p| p.p_role == ParticipantRole::Landlord && p.is_main.unwrap_or(false));
    let has_main_tenant = payload.participants.iter().any(|p| p.p_role == ParticipantRole::Tenant && p.is_main.unwrap_or(false));

    if !has_main_landlord || !has_main_tenant {
        return Err((StatusCode::BAD_REQUEST, "Se requiere al menos un Locador principal y un Locatario principal".to_string()));
    }

    let mut tx = pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let contract = sqlx::query_as::<_, Contract>(
        r#"
        INSERT INTO contracts (
            tenant_id, property_id, start_date, end_date, original_rent_amount, current_rent_amount, rent_amount, 
            adjustment_method, adjustment_frequency, automation_mode, fixed_percentage, first_notification_days,
            c_type, c_destination, jurisdiction, city, province, currency, deposit_amount, commission_amount, fees_amount,
            taxes_payer, services_payer, observations, created_by, updated_by
        )
        VALUES ($1, $2, $3, $4, $5, $5, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $23)
        RETURNING *
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
    .bind(payload.c_type.clone())
    .bind(payload.c_destination.clone())
    .bind(payload.jurisdiction.clone())
    .bind(payload.city.clone())
    .bind(payload.province.clone())
    .bind(payload.currency.clone())
    .bind(payload.deposit_amount)
    .bind(payload.commission_amount)
    .bind(payload.fees_amount)
    .bind(payload.taxes_payer.clone())
    .bind(payload.services_payer.clone())
    .bind(payload.observations.clone())
    .bind(claims.sub)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Error insertando contrato V2 en BD: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Error creando contrato".to_string())
    })?;

    // Insert participants and guarantees
    for participant_dto in payload.participants {
        let participant_id = uuid::Uuid::new_v4();
        
        sqlx::query(
            r#"
            INSERT INTO contract_participants (
                id, tenant_id, contract_id, client_id, p_role, percentage, is_main, display_order, observations, created_by, updated_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $10)
            "#
        )
        .bind(participant_id)
        .bind(tenant_id)
        .bind(contract.id)
        .bind(participant_dto.client_id)
        .bind(participant_dto.p_role.clone())
        .bind(participant_dto.percentage)
        .bind(participant_dto.is_main)
        .bind(participant_dto.display_order)
        .bind(participant_dto.observations.clone())
        .bind(claims.sub)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!("Error insertando participante: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Error insertando participante".to_string())
        })?;

        if let Some(guarantees) = participant_dto.guarantees {
            for guarantee_dto in guarantees {
                sqlx::query(
                    r#"
                    INSERT INTO participant_guarantees (
                        id, tenant_id, participant_id, guarantee_type, income_amount, employer, guarantee_details, observations, created_by, updated_by
                    )
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $9)
                    "#
                )
                .bind(uuid::Uuid::new_v4())
                .bind(tenant_id)
                .bind(participant_id)
                .bind(guarantee_dto.guarantee_type.clone())
                .bind(guarantee_dto.income_amount)
                .bind(guarantee_dto.employer.clone())
                .bind(guarantee_dto.guarantee_details.clone())
                .bind(guarantee_dto.observations.clone())
                .bind(claims.sub)
                .execute(&mut *tx)
                .await
                .map_err(|e| {
                    tracing::error!("Error insertando garantia: {}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, "Error insertando garantia".to_string())
                })?;
            }
        }
    }

    use chrono::Datelike;
    let mut current_date = payload.start_date;
    while current_date < payload.end_date {
        let mut year = current_date.year();
        let mut month = current_date.month();
        
        let due_day = if payload.start_date.day() > 10 { payload.start_date.day() } else { 10 };
        let due_date = chrono::NaiveDate::from_ymd_opt(year, month, due_day).unwrap_or(current_date);
        
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
            (StatusCode::INTERNAL_SERVER_ERROR, "Error creando cuota".to_string())
        })?;

        if month == 12 {
            year += 1;
            month = 1;
        } else {
            month += 1;
        }
        
        let next_day = if current_date.day() > 28 { 28 } else { current_date.day() };
        current_date = chrono::NaiveDate::from_ymd_opt(year, month, next_day)
            .unwrap_or_else(|| chrono::NaiveDate::from_ymd_opt(year, month, 1).unwrap());
    }

    tx.commit().await.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Error en commit".to_string()))?;

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
