use axum::{
    extract::State,
    http::StatusCode,
    routing::get,
    Extension, Json, Router,
};
use serde::Serialize;
use sqlx::PgPool;
use std::sync::Arc;
use crate::core::security::jwt::Claims;

#[derive(Serialize)]
pub struct SuperadminDashboardStats {
    pub total_tenants: i64,
    pub basic_tenants: i64,
    pub pro_tenants: i64,
    pub trial_tenants: i64,
    pub active_tenants: i64,
    pub suspended_tenants: i64,
    pub cancelled_tenants: i64,
    pub total_users: i64,
    pub mrr_estimado: f64,
}

async fn get_stats(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<SuperadminDashboardStats>, StatusCode> {
    if claims.role != "super_admin" && claims.role != "SUPERADMIN" {
        return Err(StatusCode::FORBIDDEN);
    }

    let total_tenants: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM tenants")
        .fetch_one(&*pool)
        .await
        .unwrap_or((0,));

    let total_users: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(&*pool)
        .await
        .unwrap_or((0,));

    let basic_tenants: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM subscriptions WHERE plan_type = 'BASIC'")
        .fetch_one(&*pool)
        .await
        .unwrap_or((0,));

    let pro_tenants: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM subscriptions WHERE plan_type = 'PRO'")
        .fetch_one(&*pool)
        .await
        .unwrap_or((0,));

    let trial_tenants: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM subscriptions WHERE status = 'TRIAL'")
        .fetch_one(&*pool)
        .await
        .unwrap_or((0,));

    let active_tenants: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM subscriptions WHERE status = 'ACTIVE'")
        .fetch_one(&*pool)
        .await
        .unwrap_or((0,));

    let suspended_tenants: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM subscriptions WHERE status = 'SUSPENDED'")
        .fetch_one(&*pool)
        .await
        .unwrap_or((0,));

    let cancelled_tenants: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM subscriptions WHERE status = 'CANCELLED'")
        .fetch_one(&*pool)
        .await
        .unwrap_or((0,));

    // Calculate MRR roughly based on active PRO/BASIC plans. Suppose BASIC=15000, PRO=45000 
    // This is just an estimate as requested by the spec
    let basic_price = 15000.0;
    let pro_price = 45000.0;

    let active_basic: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM subscriptions WHERE plan_type = 'BASIC' AND status = 'ACTIVE'")
        .fetch_one(&*pool)
        .await
        .unwrap_or((0,));

    let active_pro: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM subscriptions WHERE plan_type = 'PRO' AND status = 'ACTIVE'")
        .fetch_one(&*pool)
        .await
        .unwrap_or((0,));

    let mrr_estimado = (active_basic.0 as f64 * basic_price) + (active_pro.0 as f64 * pro_price);

    Ok(Json(SuperadminDashboardStats {
        total_tenants: total_tenants.0,
        basic_tenants: basic_tenants.0,
        pro_tenants: pro_tenants.0,
        trial_tenants: trial_tenants.0,
        active_tenants: active_tenants.0,
        suspended_tenants: suspended_tenants.0,
        cancelled_tenants: cancelled_tenants.0,
        total_users: total_users.0,
        mrr_estimado,
    }))
}

pub fn router(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/stats", get(get_stats))
        .with_state(pool)
}
