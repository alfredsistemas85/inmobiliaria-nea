use crate::core::security::jwt::Claims;
use axum::{extract::{Extension, Json, State}, http::StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Serialize)]
pub struct SystemSettingsResponse {
    pub saas_subscription_price: String,
}

#[derive(Deserialize)]
pub struct UpdateSystemSettingsDto {
    pub saas_subscription_price: String,
}

pub async fn get_system_settings(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<SystemSettingsResponse>, StatusCode> {
    if claims.role != "super_admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    let price_row: Option<(String,)> = sqlx::query_as("SELECT value FROM system_settings WHERE key = 'SAAS_SUBSCRIPTION_PRICE'")
        .fetch_optional(&*pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let price = price_row.map(|(v,)| v).unwrap_or_else(|| "50000".to_string());

    Ok(Json(SystemSettingsResponse {
        saas_subscription_price: price,
    }))
}

pub async fn update_system_settings(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<UpdateSystemSettingsDto>,
) -> Result<StatusCode, StatusCode> {
    if claims.role != "super_admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    sqlx::query(
        "INSERT INTO system_settings (key, value) VALUES ('SAAS_SUBSCRIPTION_PRICE', $1)
         ON CONFLICT (key) DO UPDATE SET value = EXCLUDED.value, updated_at = CURRENT_TIMESTAMP"
    )
    .bind(&payload.saas_subscription_price)
    .execute(&*pool)
    .await
    .map_err(|e| {
        tracing::error!("Error updating system settings: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(StatusCode::OK)
}
