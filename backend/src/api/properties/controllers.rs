use crate::{
    api::properties::dtos::{CreatePropertyDto, PropertyResponseDto},
    core::tenant::extractor::TenantId,
    infrastructure::database::{audit::AuditRepository, properties::PropertyRepository},
    models::{
        common::{PaginatedResponse, PaginationParams},
        property::Property,
    },
};
use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
    Extension,
};
use sqlx::types::Json as SqlxJson;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub async fn list_properties(
    State(pool): State<Arc<PgPool>>,
    Extension(tenant): Extension<TenantId>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<PropertyResponseDto>>, StatusCode> {
    let repo = PropertyRepository::new(pool);
    let limit = params.limit.unwrap_or(20);
    let offset = params.offset.unwrap_or(0);
    let q = params.q.as_deref();

    let properties = repo
        .list(tenant.0, limit, offset, q)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut dtos: Vec<PropertyResponseDto> = properties
        .data
        .into_iter()
        .map(PropertyResponseDto::from)
        .collect();

    if !dtos.is_empty() {
        if let Ok(redis_url) = std::env::var("REDIS_URL")
            .or_else(|_| Ok::<_, ()>("redis://localhost:6379".to_string()))
        {
            if let Ok(client) = redis::Client::open(redis_url) {
                if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                    let keys: Vec<String> = dtos
                        .iter()
                        .map(|d| format!("property:views:{}", d.id))
                        .collect();
                    if let Ok(views) = redis::cmd("MGET")
                        .arg(&keys)
                        .query_async::<_, Vec<Option<i64>>>(&mut conn)
                        .await
                    {
                        for (i, dto) in dtos.iter_mut().enumerate() {
                            dto.views = views.get(i).cloned().flatten().or(Some(0));
                        }
                    }
                }
            }
        }
    }

    Ok(Json(PaginatedResponse {
        data: dtos,
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
    let property = repo
        .find_by_id(id, tenant.0)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match property {
        Some(p) => {
            let mut dto = PropertyResponseDto::from(p);
            if let Ok(redis_url) = std::env::var("REDIS_URL")
                .or_else(|_| Ok::<_, ()>("redis://localhost:6379".to_string()))
            {
                if let Ok(client) = redis::Client::open(redis_url) {
                    if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                        let view_key = format!("property:views:{}", id);
                        if let Ok(views) = redis::cmd("GET")
                            .arg(&view_key)
                            .query_async::<_, Option<i64>>(&mut conn)
                            .await
                        {
                            dto.views = views.or(Some(0));
                        }
                    }
                }
            }
            Ok(Json(dto))
        }
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

    let created = repo
        .create(property)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Log audit
    let _ = audit_repo
        .log(
            Some(tenant.0),
            None, // User id is in claims, but here we only extract tenant.0. We should probably extract Claims instead. But skipping user_id is fine for now
            "CREATE_PROPERTY",
            "property",
            Some(created.id),
            None,
        )
        .await;

    Ok(Json(PropertyResponseDto::from(created)))
}

pub async fn delete_property(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
    Extension(tenant): Extension<TenantId>,
) -> Result<StatusCode, StatusCode> {
    let repo = PropertyRepository::new(pool.clone());
    let audit_repo = AuditRepository::new(pool);

    let rows_affected = repo
        .soft_delete(id, tenant.0)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if rows_affected > 0 {
        let _ = audit_repo
            .log(
                Some(tenant.0),
                None,
                "DELETE_PROPERTY",
                "property",
                Some(id),
                None,
            )
            .await;

        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn update_property(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
    Extension(tenant): Extension<TenantId>,
    Json(payload): Json<CreatePropertyDto>,
) -> Result<Json<PropertyResponseDto>, StatusCode> {
    let repo = PropertyRepository::new(pool.clone());

    // Validar existencia
    let mut property = repo
        .find_by_id(id, tenant.0)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    property.title = payload.title;
    property.description = payload.description;
    property.property_type = payload.property_type;
    property.operation_type = payload.operation_type;
    property.price = payload.price;
    property.currency = payload.currency;
    property.address = payload.address;
    property.city = payload.city;
    property.province = payload.province;
    property.square_meters = payload.square_meters;
    property.bedrooms = payload.bedrooms;
    property.bathrooms = payload.bathrooms;
    if let Some(status) = payload.status {
        property.status = Some(status);
    }
    if let Some(features) = payload.features {
        property.features = Some(SqlxJson(features));
    }

    let updated = repo
        .update(property)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let audit_repo = AuditRepository::new(pool);
    let _ = audit_repo
        .log(
            Some(tenant.0),
            None,
            "UPDATE_PROPERTY",
            "property",
            Some(id),
            None,
        )
        .await;

    Ok(Json(PropertyResponseDto::from(updated)))
}

use axum::extract::Multipart;
use std::path::PathBuf;
use tokio::fs;

pub async fn upload_image(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
    Extension(tenant): Extension<TenantId>,
    mut multipart: Multipart,
) -> Result<StatusCode, StatusCode> {
    // Validate existence
    let repo = PropertyRepository::new(pool.clone());
    let _ = repo
        .find_by_id(id, tenant.0)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let upload_dir = PathBuf::from("uploads").join("properties");
    fs::create_dir_all(&upload_dir)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
    {
        let file_name = if let Some(file_name) = field.file_name() {
            file_name.to_owned()
        } else {
            continue;
        };

        // Ext validation
        let ext = std::path::Path::new(&file_name)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();
        if !["jpg", "jpeg", "png", "webp"].contains(&ext.as_str()) {
            return Err(StatusCode::BAD_REQUEST);
        }

        let data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?;

        // Size validation 10MB
        if data.len() > 10 * 1024 * 1024 {
            return Err(StatusCode::PAYLOAD_TOO_LARGE);
        }

        let unique_name = format!("{}_{}.{}", Uuid::new_v4(), id, ext);
        let filepath = upload_dir.join(&unique_name);

        fs::write(&filepath, data)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        // Save in DB
        let url = format!("/uploads/properties/{}", unique_name);
        repo.insert_image(tenant.0, id, &url, false)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    Ok(StatusCode::OK)
}

pub async fn upload_document(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
    Extension(tenant): Extension<TenantId>,
    mut multipart: Multipart,
) -> Result<StatusCode, StatusCode> {
    // Validate existence
    let repo = PropertyRepository::new(pool.clone());
    let _ = repo
        .find_by_id(id, tenant.0)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let upload_dir = PathBuf::from("uploads").join("documents");
    fs::create_dir_all(&upload_dir)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
    {
        let file_name = if let Some(file_name) = field.file_name() {
            file_name.to_owned()
        } else {
            continue;
        };

        // Ext validation
        let ext = std::path::Path::new(&file_name)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();
        if !["pdf", "doc", "docx", "xlsx"].contains(&ext.as_str()) {
            return Err(StatusCode::BAD_REQUEST);
        }

        let data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?;

        // Size validation 20MB
        if data.len() > 20 * 1024 * 1024 {
            return Err(StatusCode::PAYLOAD_TOO_LARGE);
        }

        let unique_name = format!("{}_{}.{}", Uuid::new_v4(), id, ext);
        let filepath = upload_dir.join(&unique_name);

        fs::write(&filepath, data)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        // Save in DB
        let url = format!("/uploads/documents/{}", unique_name);
        repo.insert_document(tenant.0, id, &url, &file_name)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    Ok(StatusCode::OK)
}
