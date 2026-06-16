use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json, Extension,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;
use rust_decimal::Decimal;
use crate::core::security::jwt::Claims;

#[derive(Serialize)]
pub struct CheckoutResponse {
    pub init_point: String,
}

#[derive(Deserialize)]
pub struct WebhookPayload {
    pub action: String,
    pub data: WebhookData,
}

#[derive(Deserialize)]
pub struct WebhookData {
    pub id: String,
}

// 1. Cobro de Suscripción SaaS (SuperAdmin cobra a Inmobiliaria)
pub async fn create_subscription_preference(
    State(_pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<CheckoutResponse>, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::BAD_REQUEST)?;
    
    // Aquí iría el Access Token del SuperAdmin (dueño del SaaS)
    let access_token = std::env::var("MP_ACCESS_TOKEN").unwrap_or_default();
    
    let client = reqwest::Client::new();
    let res = client.post("https://api.mercadopago.com/checkout/preferences")
        .bearer_auth(access_token)
        .json(&json!({
            "items": [
                {
                    "title": "Suscripción SaaS InmobiCRM",
                    "quantity": 1,
                    "unit_price": 50000,
                    "currency_id": "ARS"
                }
            ],
            "external_reference": format!("SUBSCRIPTION_{}", tenant_id),
            "back_urls": {
                "success": "https://inmonea.agentech.ar/settings/billing/success",
                "failure": "https://inmonea.agentech.ar/settings/billing/failure",
                "pending": "https://inmonea.agentech.ar/settings/billing/pending"
            },
            "auto_return": "approved"
        }))
        .send()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mp_response: serde_json::Value = res.json().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if let Some(init_point) = mp_response["init_point"].as_str() {
        Ok(Json(CheckoutResponse {
            init_point: init_point.to_string(),
        }))
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

// 2. Cobro de Alquiler (Inmobiliaria cobra a Inquilino)
pub async fn create_rent_preference(
    State(pool): State<Arc<PgPool>>,
    Path(invoice_id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<CheckoutResponse>, StatusCode> {
    let tenant_id = claims.tenant_id.ok_or(StatusCode::BAD_REQUEST)?;

    #[derive(sqlx::FromRow)]
    struct InvoiceAmount { amount: Decimal }

    // Buscar la factura
    let invoice = sqlx::query_as::<_, InvoiceAmount>(
        "SELECT amount FROM invoices WHERE id = $1 AND tenant_id = $2"
    )
    .bind(invoice_id)
    .bind(tenant_id)
    .fetch_optional(&*pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    #[derive(sqlx::FromRow)]
    struct TenantToken { mp_access_token: Option<String> }

    // Buscar credenciales MP del Tenant
    let tenant = sqlx::query_as::<_, TenantToken>("SELECT mp_access_token FROM tenants WHERE id = $1")
        .bind(tenant_id)
        .fetch_one(&*pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let access_token = tenant.mp_access_token.ok_or(StatusCode::BAD_REQUEST)?;

    let client = reqwest::Client::new();
    let res = client.post("https://api.mercadopago.com/checkout/preferences")
        .bearer_auth(access_token)
        .json(&json!({
            "items": [
                {
                    "title": "Pago de Alquiler / Expensas",
                    "quantity": 1,
                    "unit_price": invoice.amount,
                    "currency_id": "ARS"
                }
            ],
            "external_reference": format!("RENT_{}", invoice_id),
            "back_urls": {
                "success": "https://inmonea.agentech.ar/payments/success",
                "failure": "https://inmonea.agentech.ar/payments/failure",
                "pending": "https://inmonea.agentech.ar/payments/pending"
            },
            "auto_return": "approved"
        }))
        .send()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mp_response: serde_json::Value = res.json().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if let Some(init_point) = mp_response["init_point"].as_str() {
        Ok(Json(CheckoutResponse {
            init_point: init_point.to_string(),
        }))
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

// 3. Webhook de Mercado Pago
pub async fn mp_webhook(
    State(_pool): State<Arc<PgPool>>,
    Json(payload): Json<WebhookPayload>,
) -> Result<StatusCode, StatusCode> {
    if payload.action == "payment.created" || payload.action == "payment.updated" {
        // En producción, aquí se consulta a MP por el ID del pago para verificar status y external_reference
        // Por simplificación en el código base, asumimos que se consulta:
        let payment_id = payload.data.id.clone();
        
        // Simulación:
        tracing::info!("Webhook recibido para payment_id: {}", payment_id);
        
        // Si el pago es de alquiler, se actualizaría la invoice y se crearía el Payment
        // Si es de SaaS, se actualizaría la suscripción del tenant
    }

    Ok(StatusCode::OK)
}
