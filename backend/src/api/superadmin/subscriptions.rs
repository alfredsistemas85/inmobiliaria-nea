use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, patch},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;
use crate::core::security::jwt::Claims;

#[derive(Debug, Deserialize)]
pub struct ChangePlanDto {
    pub plan_type: String, // "BASIC" or "PRO"
}

#[derive(Debug, Deserialize)]
pub struct SetStatusDto {
    pub status: String, // "ACTIVE", "SUSPENDED", "CANCELLED"
}

#[derive(Debug, Deserialize)]
pub struct ExtendTrialDto {
    pub days_to_add: Option<u32>,
    pub trial_ends_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize)]
pub struct SubscriptionResponse {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub plan_type: String,
    pub status: String,
    pub trial_ends_at: Option<chrono::DateTime<chrono::Utc>>,
}

async fn list_subscriptions(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<SubscriptionResponse>>, StatusCode> {
    if claims.role != "super_admin" && claims.role != "SUPERADMIN" {
        return Err(StatusCode::FORBIDDEN);
    }

    let records = sqlx::query!(
        "SELECT id, tenant_id, plan_type::text as plan_type, status::text as status, trial_ends_at FROM subscriptions"
    )
    .fetch_all(&*pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut response = Vec::new();
    for r in records {
        response.push(SubscriptionResponse {
            id: r.id,
            tenant_id: r.tenant_id,
            plan_type: r.plan_type.unwrap_or_default(),
            status: r.status.unwrap_or_default(),
            trial_ends_at: r.trial_ends_at,
        });
    }

    Ok(Json(response))
}

async fn get_subscription(
    State(pool): State<Arc<PgPool>>,
    Path(tenant_id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<SubscriptionResponse>, StatusCode> {
    if claims.role != "super_admin" && claims.role != "SUPERADMIN" {
        return Err(StatusCode::FORBIDDEN);
    }

    let record = sqlx::query!(
        "SELECT id, tenant_id, plan_type::text as plan_type, status::text as status, trial_ends_at FROM subscriptions WHERE tenant_id = $1",
        tenant_id
    )
    .fetch_optional(&*pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(SubscriptionResponse {
        id: record.id,
        tenant_id: record.tenant_id,
        plan_type: record.plan_type.unwrap_or_default(),
        status: record.status.unwrap_or_default(),
        trial_ends_at: record.trial_ends_at,
    }))
}

async fn change_plan(
    State(pool): State<Arc<PgPool>>,
    Path(tenant_id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<ChangePlanDto>,
) -> Result<StatusCode, StatusCode> {
    if claims.role != "super_admin" && claims.role != "SUPERADMIN" {
        return Err(StatusCode::FORBIDDEN);
    }

    let plan_enum = match payload.plan_type.as_str() {
        "BASIC" => "BASIC",
        "PRO" => "PRO",
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    let mut tx = pool.begin().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query(
        "UPDATE subscriptions SET plan_type = $1::plan_type WHERE tenant_id = $2"
    )
    .bind(plan_enum)
    .bind(tenant_id)
    .execute(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query(
        r#"INSERT INTO audit_logs (tenant_id, user_id, action, entity_type, entity_id, new_data)
           VALUES ($1, $2, 'SUBSCRIPTION_CHANGED', 'subscription', $1, $3)"#,
    )
    .bind(tenant_id)
    .bind(claims.sub)
    .bind(serde_json::json!({ "new_plan": plan_enum }))
    .execute(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    tx.commit().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

async fn set_status(
    State(pool): State<Arc<PgPool>>,
    Path(tenant_id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<SetStatusDto>,
) -> Result<StatusCode, StatusCode> {
    if claims.role != "super_admin" && claims.role != "SUPERADMIN" {
        return Err(StatusCode::FORBIDDEN);
    }

    let status_enum = match payload.status.as_str() {
        "ACTIVE" => "ACTIVE",
        "SUSPENDED" => "SUSPENDED",
        "CANCELLED" => "CANCELLED",
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    let action_str = match status_enum {
        "SUSPENDED" => "SUBSCRIPTION_SUSPENDED",
        "CANCELLED" => "SUBSCRIPTION_CANCELLED",
        _ => "SUBSCRIPTION_CHANGED",
    };

    let mut tx = pool.begin().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query(
        "UPDATE subscriptions SET status = $1::subscription_status WHERE tenant_id = $2"
    )
    .bind(status_enum)
    .bind(tenant_id)
    .execute(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query(
        r#"INSERT INTO audit_logs (tenant_id, user_id, action, entity_type, entity_id, new_data)
           VALUES ($1, $2, $3, 'subscription', $1, $4)"#,
    )
    .bind(tenant_id)
    .bind(claims.sub)
    .bind(action_str)
    .bind(serde_json::json!({ "new_status": status_enum }))
    .execute(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    tx.commit().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

async fn extend_trial(
    State(pool): State<Arc<PgPool>>,
    Path(tenant_id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<ExtendTrialDto>,
) -> Result<StatusCode, StatusCode> {
    if claims.role != "super_admin" && claims.role != "SUPERADMIN" {
        return Err(StatusCode::FORBIDDEN);
    }

    let new_trial_date = if let Some(d) = payload.trial_ends_at {
        d
    } else if let Some(days) = payload.days_to_add {
        chrono::Utc::now() + chrono::Duration::days(days as i64)
    } else {
        return Err(StatusCode::BAD_REQUEST);
    };

    let mut tx = pool.begin().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query(
        "UPDATE subscriptions SET trial_ends_at = $1, status = 'TRIAL'::subscription_status WHERE tenant_id = $2"
    )
    .bind(new_trial_date)
    .bind(tenant_id)
    .execute(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query(
        r#"INSERT INTO audit_logs (tenant_id, user_id, action, entity_type, entity_id, new_data)
           VALUES ($1, $2, 'SUBSCRIPTION_CHANGED', 'subscription', $1, $3)"#,
    )
    .bind(tenant_id)
    .bind(claims.sub)
    .bind(serde_json::json!({ "trial_extended_to": new_trial_date }))
    .execute(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    tx.commit().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

pub fn router(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/", get(list_subscriptions))
        .route("/:tenant_id", get(get_subscription))
        .route("/:tenant_id/plan", patch(change_plan))
        .route("/:tenant_id/status", patch(set_status))
        .route("/:tenant_id/trial", patch(extend_trial))
        .with_state(pool)
}
