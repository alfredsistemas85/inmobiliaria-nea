use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json, Extension,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;
use crate::core::security::jwt::Claims;
use rust_decimal::Decimal;

#[derive(Serialize, sqlx::FromRow)]
pub struct Invoice {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub contract_id: Option<Uuid>,
    pub amount: Decimal,
    pub commission: Option<Decimal>,
    pub status: Option<String>,
    pub due_date: NaiveDate,
}

#[derive(Deserialize)]
pub struct CreateInvoiceDto {
    pub contract_id: Uuid,
    pub amount: Decimal,
    pub commission: Decimal,
    pub due_date: NaiveDate,
}

#[derive(Serialize)]
pub struct Liquidation {
    pub owner_name: String,
    pub property_title: String,
    pub total_collected: Decimal,
    pub commission_deducted: Decimal,
    pub net_to_transfer: Decimal,
}

pub async fn list_invoices(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<Invoice>>, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::BAD_REQUEST)?;

    let invoices = sqlx::query_as::<_, Invoice>(
        "SELECT id, tenant_id, contract_id, amount, commission, status, due_date FROM invoices WHERE tenant_id = $1 ORDER BY due_date DESC"
    )
    .bind(tenant_id)
    .fetch_all(&*pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(invoices))
}

pub async fn create_invoice(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateInvoiceDto>,
) -> Result<Json<Invoice>, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::BAD_REQUEST)?;

    let invoice = sqlx::query_as::<_, Invoice>(
        r#"
        INSERT INTO invoices (tenant_id, contract_id, amount, commission, due_date)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, tenant_id, contract_id, amount, commission, status, due_date
        "#
    )
    .bind(tenant_id)
    .bind(payload.contract_id)
    .bind(payload.amount)
    .bind(payload.commission)
    .bind(payload.due_date)
    .fetch_one(&*pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(invoice))
}

pub async fn mark_invoice_paid(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> Result<StatusCode, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::BAD_REQUEST)?;

    let rows_affected = sqlx::query(
        "UPDATE invoices SET status = 'PAID' WHERE id = $1 AND tenant_id = $2"
    )
    .bind(id)
    .bind(tenant_id)
    .execute(&*pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .rows_affected();

    if rows_affected == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::OK)
}

#[derive(sqlx::FromRow)]
struct LiquidationRecord {
    amount: Decimal,
    commission: Option<Decimal>,
    property_title: String,
    owner_name: Option<String>,
}

pub async fn generate_liquidations(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<Liquidation>>, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::BAD_REQUEST)?;

    // Simplificación: Traer facturas pagadas y calcular neto
    let records = sqlx::query_as::<_, LiquidationRecord>(
        r#"
        SELECT 
            i.amount, 
            i.commission, 
            p.title as property_title, 
            c.first_name || ' ' || c.last_name as owner_name 
        FROM invoices i
        JOIN contracts ct ON i.contract_id = ct.id
        JOIN properties p ON ct.property_id = p.id
        JOIN clients c ON p.owner_id = c.id
        WHERE i.tenant_id = $1 AND i.status = 'PAID'
        "#
    )
    .bind(tenant_id)
    .fetch_all(&*pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let liquidations: Vec<Liquidation> = records.into_iter().map(|rec| {
        let amt = rec.amount;
        let comm = rec.commission.unwrap_or_default();
        Liquidation {
            owner_name: rec.owner_name.unwrap_or_default(),
            property_title: rec.property_title,
            total_collected: amt,
            commission_deducted: comm,
            net_to_transfer: amt - comm,
        }
    }).collect();

    Ok(Json(liquidations))
}
