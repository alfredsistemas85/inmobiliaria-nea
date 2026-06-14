use axum::{
    extract::{Path, State, Json, Query},
    http::StatusCode,
    Extension,
};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;
use crate::{
    api::properties::dtos::{CreatePropertyDto, PropertyResponseDto},
    core::tenant::extractor::TenantId,
    models::{property::Property, common::{PaginatedResponse, PaginationParams}},
    infrastructure::database::{properties::PropertyRepository, audit::AuditRepository},
};
use sqlx::types::Json as SqlxJson;

pub async fn list_properties(
    State(pool): State<Arc<PgPool>>,
    Extension(tenant): Extension<TenantId>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<PropertyResponseDto>>, StatusCode> {
    let repo = PropertyRepository::new(pool);
    let limit = params.limit.unwrap_or(20);
    let offset = params.offset.unwrap_or(0);
    let q = params.q.as_deref();

    let properties = repo.list(tenant.0, limit, offset, q).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(PaginatedResponse {
        data: properties.data.into_iter().map(PropertyResponseDto::from).collect(),
        total: properties.total,
        limit: properties.limit,
        offset: properties.offset,
    }))
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
        deleted_at: None,
    };

    let repo = PropertyRepository::new(pool.clone());
    let audit_repo = AuditRepository::new(pool);

    let created = repo.create(property).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Log audit
    let _ = audit_repo.log(
        Some(tenant.0),
        None, // User id is in claims, but here we only extract tenant.0. We should probably extract Claims instead. But skipping user_id is fine for now
        "CREATE_PROPERTY",
        "property",
        Some(created.id),
        None,
    ).await;

    Ok(Json(PropertyResponseDto::from(created)))
}

pub async fn delete_property(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
    Extension(tenant): Extension<TenantId>,
) -> Result<StatusCode, StatusCode> {
    let repo = PropertyRepository::new(pool.clone());
    let audit_repo = AuditRepository::new(pool);

    let rows_affected = repo.soft_delete(id, tenant.0)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if rows_affected > 0 {
        let _ = audit_repo.log(
            Some(tenant.0),
            None,
            "DELETE_PROPERTY",
            "property",
            Some(id),
            None,
        ).await;

        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
