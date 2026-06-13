use axum::{
    extract::{Path, State, Json},
    http::StatusCode,
    Extension,
};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;
use crate::{
    api::properties::dtos::{CreatePropertyDto, PropertyResponseDto},
    core::tenant::extractor::TenantId,
    models::property::Property,
    infrastructure::database::properties::PropertyRepository,
};
use sqlx::types::Json as SqlxJson;

pub async fn list_properties(
    State(pool): State<Arc<PgPool>>,
    Extension(tenant): Extension<TenantId>,
) -> Result<Json<Vec<PropertyResponseDto>>, StatusCode> {
    let properties = sqlx::query_as::<_, Property>("SELECT * FROM properties WHERE tenant_id = $1")
        .bind(tenant.0)
        .fetch_all(&*pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(properties.into_iter().map(PropertyResponseDto::from).collect()))
}

pub async fn get_property(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
    Extension(tenant): Extension<TenantId>,
) -> Result<Json<PropertyResponseDto>, StatusCode> {
    let repo = PropertyRepository::new(pool);
    let property = repo.find_by_id(id, tenant.0).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match property {
        Some(p) => Ok(Json(PropertyResponseDto::from(p))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn create_property(
    State(pool): State<Arc<PgPool>>,
    Extension(tenant): Extension<TenantId>,
    Json(payload): Json<CreatePropertyDto>,
) -> Result<Json<PropertyResponseDto>, StatusCode> {
    
    let property = Property {
        id: Uuid::new_v4(),
        tenant_id: tenant.0,
        title: payload.title,
        description: payload.description,
        property_type: payload.property_type,
        operation_type: payload.operation_type,
        price: payload.price,
        currency: payload.currency,
        address: payload.address,
        city: payload.city,
        province: payload.province,
        square_meters: payload.square_meters,
        bedrooms: payload.bedrooms,
        bathrooms: payload.bathrooms,
        status: payload.status.or(Some("Disponible".to_string())),
        features: payload.features.map(SqlxJson),
        created_at: None,
        updated_at: None,
    };

    let repo = PropertyRepository::new(pool);
    let created = repo.create(property).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(PropertyResponseDto::from(created)))
}

pub async fn delete_property(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
    Extension(tenant): Extension<TenantId>,
) -> Result<StatusCode, StatusCode> {
    let rows_affected = sqlx::query("DELETE FROM properties WHERE id = $1 AND tenant_id = $2")
        .bind(id)
        .bind(tenant.0)
        .execute(&*pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .rows_affected();

    if rows_affected > 0 {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
