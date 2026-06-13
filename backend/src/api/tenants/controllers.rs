use axum::{
    extract::{Path, State, Json},
    http::StatusCode,
    Extension,
};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;
use crate::{
    api::tenants::dtos::{CreateTenantDto, TenantResponseDto},
    models::tenant::Tenant,
    infrastructure::database::tenants::TenantRepository,
    core::security::jwt::Claims,
};

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

    Ok(Json(tenants.into_iter().map(TenantResponseDto::from).collect()))
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
    let tenant = repo.find_by_id(id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

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
    if claims.role != "super_admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    let tenant = Tenant {
        id: Uuid::new_v4(),
        cuit: payload.cuit,
        dni_responsable: payload.dni_responsable,
        first_name: payload.first_name,
        last_name: payload.last_name,
        business_name: payload.business_name,
        address: payload.address,
        phone: payload.phone,
        city: payload.city,
        province: payload.province,
        is_active: Some(true),
        created_at: None,
        updated_at: None,
    };

    let repo = TenantRepository::new(pool);
    let created = repo.create(tenant).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(TenantResponseDto::from(created)))
}
