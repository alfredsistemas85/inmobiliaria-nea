use crate::{
    api::tenants::dtos::{CreateTenantDto, TenantResponseDto},
    core::security::jwt::Claims,
    infrastructure::database::tenants::TenantRepository,
    models::tenant::Tenant,
};
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    Extension,
};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub async fn list_tenants(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<TenantResponseDto>>, StatusCode> {
    if claims.role != "super_admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    let tenants = sqlx::query_as::<_, Tenant>("SELECT * FROM tenants")
        .fetch_all(&*pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(
        tenants.into_iter().map(TenantResponseDto::from).collect(),
    ))
}

pub async fn get_tenant(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<TenantResponseDto>, StatusCode> {
    if claims.role != "super_admin" && claims.tenant_id != Some(id) {
        return Err(StatusCode::FORBIDDEN);
    }

    let repo = TenantRepository::new(pool);
    let tenant = repo
        .find_by_id(id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match tenant {
        Some(t) => Ok(Json(TenantResponseDto::from(t))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn create_tenant(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateTenantDto>,
) -> Result<Json<TenantResponseDto>, StatusCode> {
    if claims.role != "super_admin" && claims.role != "SUPERADMIN" {
        return Err(StatusCode::FORBIDDEN);
    }

    let normalized_cuit = crate::core::utils::cuit_validator::normalize_cuit(&payload.cuit);
    if !crate::core::utils::cuit_validator::is_valid_cuit(&normalized_cuit) {
        tracing::warn!("CREATE_TENANT_FAILED: invalid CUIT {}", payload.cuit);
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut tx = pool
        .begin()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let tenant_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
    let onboarding_token = Uuid::new_v4().to_string();
    let trial_ends_at = chrono::Utc::now() + chrono::Duration::days(15);
    let onboarding_expires = chrono::Utc::now() + chrono::Duration::hours(48);

    // 1. Create Tenant
    sqlx::query(
        r#"INSERT INTO tenants (id, cuit, dni_responsable, first_name, last_name, business_name, phone, is_active, status)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"#,
    )
    .bind(tenant_id)
    .bind(&normalized_cuit)
    .bind("") // Dni responsable is no longer in DTO, fallback to empty
    .bind(&payload.admin_first_name)
    .bind(&payload.admin_last_name)
    .bind(&payload.business_name)
    .bind(&payload.phone)
    .bind(true)
    .bind("ACTIVE")
    .execute(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 2. Create Subscription (BASIC / TRIAL)
    sqlx::query(
        r#"INSERT INTO subscriptions (tenant_id, plan_type, status, trial_ends_at)
           VALUES ($1, 'BASIC', 'TRIAL', $2)"#,
    )
    .bind(tenant_id)
    .bind(trial_ends_at)
    .execute(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 3. Create Admin User
    sqlx::query(
        r#"INSERT INTO users (id, tenant_id, role, email, password_hash, first_name, last_name, is_active, onboarding_token, onboarding_token_expires_at)
           VALUES ($1, $2, 'ADMIN_INMOBILIARIA', $3, '', $4, $5, true, $6, $7)"#,
    )
    .bind(user_id)
    .bind(tenant_id)
    .bind(&payload.admin_email)
    .bind(&payload.admin_first_name)
    .bind(&payload.admin_last_name)
    .bind(&onboarding_token)
    .bind(onboarding_expires)
    .execute(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 4. Log Audit
    sqlx::query(
        r#"INSERT INTO audit_logs (tenant_id, user_id, action, entity_type, entity_id, new_data)
           VALUES ($1, $2, 'TENANT_CREATED', 'tenant', $1, $3)"#,
    )
    .bind(tenant_id)
    .bind(claims.sub)
    .bind(serde_json::json!({ "business_name": payload.business_name, "cuit": payload.cuit }))
    .execute(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    tx.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    tracing::info!(
        "ONBOARDING EMAIL SIMULADO: Enviar a {} token: {}",
        payload.admin_email,
        onboarding_token
    );

    let response = TenantResponseDto {
        id: tenant_id,
        cuit: normalized_cuit,
        dni_responsable: "".to_string(),
        first_name: payload.admin_first_name,
        last_name: payload.admin_last_name,
        business_name: payload.business_name,
        is_active: Some(true),
    };

    Ok(Json(response))
}

#[derive(serde::Deserialize)]
pub struct UpdateTenantStatusDto {
    pub status: String,
}

pub async fn update_tenant_status(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<UpdateTenantStatusDto>,
) -> Result<StatusCode, StatusCode> {
    if claims.role != "super_admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    let valid_statuses = ["PENDING", "ACTIVE", "SUSPENDED", "DELETED"];
    if !valid_statuses.contains(&payload.status.as_str()) {
        return Err(StatusCode::BAD_REQUEST);
    }

    let repo = TenantRepository::new(pool);
    repo.update_status(id, &payload.status)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}
