use super::dto::{ProposeAdjustmentDto, ApproveAdjustmentDto, RollbackAdjustmentDto};
use crate::api::contracts::models::{RentAdjustment, ContractInstallment};
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
    // Engine call placeholder
    Ok(StatusCode::CREATED)
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

pub async fn rollback_adjustment(
    State(pool): State<Arc<PgPool>>,
    Path(adj_id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<RollbackAdjustmentDto>,
) -> Result<StatusCode, StatusCode> {
    let engine = RentalAdjustmentEngine::new(pool);
    
    engine.rollback_adjustment(adj_id, claims.sub, payload.reason)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
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
