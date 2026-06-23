use super::dto::{ProposeAdjustmentDto, ApproveAdjustmentDto, RollbackAdjustmentDto};
use crate::api::contracts::models::{RentAdjustment, ContractInstallment};
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use chrono::NaiveDate;
use crate::core::contracts::adjustment_engine::RentalAdjustmentEngine;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json, Extension,
};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;
use crate::core::security::jwt::Claims;

// This would be injected via AppState in a real scenario
// For now, we instantiate it directly or expect it in State

pub async fn list_adjustments(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<RentAdjustment>>, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::BAD_REQUEST)?;

    let adjustments = sqlx::query_as::<_, RentAdjustment>(
        "SELECT id, tenant_id, contract_id, adjustment_method, status, previous_amount, new_amount, percentage_applied, index_name, index_initial_value, index_final_value, index_snapshot, rollback_reason, approved_by, approved_at, effective_date, notes, created_at FROM rent_adjustments WHERE contract_id = $1 AND tenant_id = $2 ORDER BY created_at DESC"
    )
    .bind(id)
    .bind(tenant_id)
    .fetch_all(&*pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(adjustments))
}

pub async fn propose_adjustment(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
    // payload for manual/specific date
) -> Result<StatusCode, StatusCode> {
    // INC-022: TODO — Implement adjustment proposal logic via RentalAdjustmentEngine
    // This endpoint is not yet functional. Return 501 Not Implemented.
    tracing::warn!("propose_adjustment called but not yet implemented for contract_id={}", id);
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn approve_adjustment(
    State(pool): State<Arc<PgPool>>,
    Path(adj_id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<ApproveAdjustmentDto>,
) -> Result<StatusCode, StatusCode> {
    let engine = RentalAdjustmentEngine::new(pool);
    
    engine.approve_adjustment(adj_id, claims.sub, payload.new_amount, payload.notes)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

pub async fn reject_adjustment(
    State(pool): State<Arc<PgPool>>,
    Path(adj_id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<RollbackAdjustmentDto>, // Reusing the same dto shape with `reason`
) -> Result<StatusCode, StatusCode> {
    let engine = RentalAdjustmentEngine::new(pool);
    
    engine.reject_adjustment(adj_id, claims.sub, payload.reason)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

#[derive(Serialize)]
pub struct PendingAdjustmentDto {
    pub adjustment_id: Uuid,
    pub contract_id: Uuid,
    pub contract_number: String,
    pub tenant_name: String,
    pub current_rent: Option<Decimal>,
    pub adjustment_percent: Option<Decimal>,
    pub new_rent: Option<Decimal>,
    pub effective_date: Option<NaiveDate>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize)]
pub struct PendingAdjustmentsResponse {
    pub items: Vec<PendingAdjustmentDto>,
    pub total: i64,
}

pub async fn list_pending_adjustments(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<PendingAdjustmentsResponse>, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::BAD_REQUEST)?;

    #[derive(sqlx::FromRow)]
    struct PendingRecord {
        adjustment_id: Uuid,
        contract_id: Uuid,
        contract_number: String,
        tenant_name: Option<String>,
        current_rent: Option<Decimal>,
        adjustment_percent: Option<Decimal>,
        new_rent: Option<Decimal>,
        effective_date: Option<NaiveDate>,
        created_at: Option<chrono::DateTime<chrono::Utc>>,
    }

    let records = sqlx::query_as::<_, PendingRecord>(
        r#"
        SELECT 
            ra.id as adjustment_id,
            ra.contract_id,
            c.id::text as contract_number,
            COALESCE(cl.first_name || ' ' || cl.last_name, '') as tenant_name,
            ra.previous_amount as current_rent,
            ra.percentage_applied as adjustment_percent,
            ra.new_amount as new_rent,
            ra.effective_date,
            ra.created_at
        FROM rent_adjustments ra
        JOIN contracts c ON ra.contract_id = c.id
        LEFT JOIN clients cl ON c.tenant_user_id = cl.id
        WHERE ra.tenant_id = $1 AND ra.status = 'PENDING'
        ORDER BY ra.effective_date ASC
        "#
    )
    .bind(tenant_id)
    .fetch_all(&*pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let items = records.into_iter().map(|r| PendingAdjustmentDto {
        adjustment_id: r.adjustment_id,
        contract_id: r.contract_id,
        contract_number: r.contract_number,
        tenant_name: r.tenant_name.unwrap_or_default(),
        current_rent: r.current_rent,
        adjustment_percent: r.adjustment_percent,
        new_rent: r.new_rent,
        effective_date: r.effective_date,
        created_at: r.created_at,
    }).collect::<Vec<_>>();

    let total = items.len() as i64;

    Ok(Json(PendingAdjustmentsResponse { items, total }))
}

pub async fn list_installments(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<ContractInstallment>>, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::BAD_REQUEST)?;

    let installments = sqlx::query_as::<_, ContractInstallment>(
        "SELECT id, tenant_id, contract_id, due_date, amount, status, created_at, updated_at FROM contract_installments WHERE contract_id = $1 AND tenant_id = $2 ORDER BY due_date ASC"
    )
    .bind(id)
    .bind(tenant_id)
    .fetch_all(&*pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(installments))
}
