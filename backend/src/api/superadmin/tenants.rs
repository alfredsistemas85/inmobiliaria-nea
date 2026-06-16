use crate::{
    core::rbac::middleware::require_super_admin,
    infrastructure::database::tenants::TenantRepository,
    models::tenant::Tenant,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    middleware,
    routing::{get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Serialize)]
pub struct TenantListItem {
    pub id: Uuid,
    pub business_name: String,
    pub cuit: String,
    pub phone: Option<String>,
    pub status: Option<String>,
    pub is_active: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct CreateTenantDto {
    pub business_name: String,
    pub cuit: String,
    pub dni_responsable: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateTenantStatusDto {
    pub status: String, // 'ACTIVE', 'PENDING', 'SUSPENDED', 'DELETED'
}

async fn list_tenants(
    State(pool): State<Arc<PgPool>>,
) -> Result<Json<Vec<TenantListItem>>, StatusCode> {
    let tenants = sqlx::query_as!(
        TenantListItem,
        r#"
        SELECT id, business_name, cuit, phone, status, is_active, created_at
        FROM tenants
        WHERE status != 'DELETED' OR status IS NULL
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| {
        tracing::error!("Error listing tenants: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(tenants))
}

async fn get_tenant(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Tenant>, StatusCode> {
    let repo = TenantRepository::new(pool);
    let tenant = repo
        .find_by_id(id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(tenant))
}

async fn create_tenant(
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<CreateTenantDto>,
) -> Result<Json<Tenant>, axum::response::Response> {
    let mut slug = payload.business_name.to_lowercase().replace(" ", "-");
    slug.retain(|c| c.is_ascii_alphanumeric() || c == '-');
    
    let tenant = match sqlx::query_as!(
        Tenant,
        r#"
        INSERT INTO tenants (business_name, cuit, dni_responsable, first_name, last_name, phone, status, slug, is_active)
        VALUES ($1, $2, $3, $4, $5, $6, 'PENDING', $7, false)
        RETURNING *
        "#,
        payload.business_name,
        payload.cuit,
        payload.dni_responsable,
        payload.first_name,
        payload.last_name,
        payload.phone,
        slug
    )
    .fetch_one(&*pool)
    .await
    {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("Error creating tenant: {}", e);
            if let Some(db_err) = e.as_database_error() {
                let msg = db_err.message();
                if msg.contains("tenants_cuit_key") || msg.contains("cuit") {
                    let err_resp = (StatusCode::CONFLICT, axum::Json(serde_json::json!({"error": "El CUIT ingresado ya está registrado"}))).into_response();
                    return Err(err_resp);
                } else if msg.contains("tenants_slug_key") || msg.contains("slug") {
                    let err_resp = (StatusCode::CONFLICT, axum::Json(serde_json::json!({"error": "El nombre de inmobiliaria genera un identificador ya en uso"}))).into_response();
                    return Err(err_resp);
                }
            }
            let err_resp = (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(serde_json::json!({"error": "Error interno del servidor"}))).into_response();
            return Err(err_resp);
        }
    };

    Ok(Json(tenant))
}

async fn update_tenant_status(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTenantStatusDto>,
) -> Result<StatusCode, StatusCode> {
    let is_active = payload.status == "ACTIVE";

    let result = sqlx::query!(
        "UPDATE tenants SET status = $1, is_active = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3",
        payload.status,
        is_active,
        id
    )
    .execute(&*pool)
    .await
    .map_err(|e| {
        tracing::error!("Error updating tenant status: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    tracing::info!("TENANT_STATUS_UPDATED: tenant_id={} new_status={}", id, payload.status);

    Ok(StatusCode::OK)
}

pub fn router(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/", get(list_tenants).post(create_tenant))
        .route("/:id", get(get_tenant))
        .route("/:id/status", put(update_tenant_status))
        .route_layer(middleware::from_fn(require_super_admin))
        .with_state(pool)
}
