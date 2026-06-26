use crate::core::contracts::adjustment_engine::RentalAdjustmentEngine;
use crate::core::system_errors::AppError;
use crate::core::workers::adjustment_scheduler::RentalAdjustmentScheduler;
use axum::http::StatusCode;
use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use serde_json::json;
use sqlx::PgPool;
use std::sync::Arc;

pub fn router(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/trigger-adjustments", post(trigger_adjustments))
        .with_state(pool)
}

pub async fn trigger_adjustments(
    State(pool): State<Arc<PgPool>>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    // Audit execution
    let _ = sqlx::query(
        "INSERT INTO audit_logs (tenant_id, user_id, action, old_data, new_data, method)
         VALUES ($1, $2, 'MANUAL_TRIGGER_EXECUTED', '{}', '{}', 'POST')",
    )
    .bind(uuid::Uuid::nil()) // Or superadmin ID if available
    .bind(uuid::Uuid::nil())
    .execute(&*pool)
    .await;

    // Trigger Scheduler
    let engine = Arc::new(RentalAdjustmentEngine::new(pool.clone()));
    let scheduler = RentalAdjustmentScheduler::new(pool.clone(), engine);

    match scheduler.process_daily_adjustments().await {
        Ok(metrics) => Ok((StatusCode::OK, Json(serde_json::to_value(metrics).unwrap()))),
        Err(e) => {
            tracing::error!("Manual adjustment trigger failed: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
