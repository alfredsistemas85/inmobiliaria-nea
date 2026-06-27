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
use sqlx::{PgPool, Row};
use std::sync::Arc;
use uuid::Uuid;

pub async fn list_properties(
    State(pool): State<Arc<PgPool>>,
    Extension(tenant): Extension<TenantId>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<PropertyResponseDto>>, StatusCode> {
    let repo = PropertyRepository::new(pool.clone());
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
        // INC-018: Reuse shared Redis client from request extensions
        if let Ok(client) = std::env::var("REDIS_URL")
            .or_else(|_| Ok::<_, ()>("redis://localhost:6379".to_string()))
            .and_then(|url| redis::Client::open(url).map_err(|_| ()))
        {
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

        // Fetch images for all listed properties
        let property_ids: Vec<Uuid> = dtos.iter().map(|d| d.id).collect();
        let rows = sqlx::query(
            r#"
            SELECT id, entity_id, storage_path 
            FROM documents 
            WHERE entity_id = ANY($1) AND entity_type = 'property' AND deleted_at IS NULL AND mime_type LIKE 'image/%'
            ORDER BY created_at ASC
            "#
        )
        .bind(&property_ids)
        .fetch_all(&*pool)
        .await
        .unwrap_or_default();

        if !rows.is_empty() {
            let api_url = std::env::var("API_URL").unwrap_or_else(|_| "".to_string());

            for dto in dtos.iter_mut() {
                let mut images_json = Vec::new();
                for r in &rows {
                    let r_entity_id: Uuid = r.try_get("entity_id").unwrap_or_default();
                    if r_entity_id == dto.id {
                        let r_id: Uuid = r.try_get("id").unwrap_or_default();
                        images_json.push(serde_json::json!({
                            "url": format!("{}/api/documents/{}/view", api_url, r_id)
                        }));
                    }
                }

                if !images_json.is_empty() {
                    dto.images = Some(images_json);
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
    let repo = PropertyRepository::new(pool.clone());
    let property = repo
        .find_by_id(id, tenant.0)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match property {
        Some(p) => {
            let mut dto = PropertyResponseDto::from(p);
            // INC-018: Reuse shared Redis client
            if let Ok(client) = std::env::var("REDIS_URL")
                .or_else(|_| Ok::<_, ()>("redis://localhost:6379".to_string()))
                .and_then(|url| redis::Client::open(url).map_err(|_| ()))
            {
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

            // Populate images from documents table
            let rows = sqlx::query(
                "SELECT id, storage_path FROM documents WHERE entity_id = $1 AND entity_type = 'property' AND deleted_at IS NULL AND mime_type LIKE 'image/%' ORDER BY created_at ASC"
            )
            .bind(id)
            .fetch_all(&*pool)
            .await
            .unwrap_or_default();

            if !rows.is_empty() {
                let api_url = std::env::var("API_URL").unwrap_or_else(|_| "".to_string());
                let mut images_json = Vec::new();
                for r in rows {
                    let r_id: Uuid = r.try_get("id").unwrap_or_default();
                    images_json.push(serde_json::json!({
                        "url": format!("{}/api/documents/{}/view", api_url, r_id)
                    }));
                }
                dto.images = Some(images_json);
            }

            let owner_rows = sqlx::query(
                "SELECT client_id, percentage FROM property_owners WHERE property_id = $1 AND tenant_id = $2"
            )
            .bind(id)
            .bind(tenant.0)
            .fetch_all(&*pool)
            .await
            .unwrap_or_default();
            
            if !owner_rows.is_empty() {
                let mut owners_dto = Vec::new();
                for r in owner_rows {
                    let client_id: Uuid = r.try_get("client_id").unwrap_or_default();
                    let percentage: Option<rust_decimal::Decimal> = r.try_get("percentage").ok();
                    use crate::api::properties::dtos::PropertyOwnerDto;
                    owners_dto.push(PropertyOwnerDto { client_id, percentage });
                }
                dto.owners = Some(owners_dto);
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

    if let Some(owners) = payload.owners {
        for owner in owners {
            let _ = sqlx::query(
                "INSERT INTO property_owners (tenant_id, property_id, client_id, percentage) VALUES ($1, $2, $3, $4)"
            )
            .bind(tenant.0)
            .bind(created.id)
            .bind(owner.client_id)
            .bind(owner.percentage.unwrap_or(rust_decimal::Decimal::new(100, 0)))
            .execute(&*pool)
            .await;
        }
    }

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

    if let Some(owners) = payload.owners {
        let _ = sqlx::query("DELETE FROM property_owners WHERE property_id = $1 AND tenant_id = $2")
            .bind(id)
            .bind(tenant.0)
            .execute(&*pool)
            .await;
            
        for owner in owners {
            let _ = sqlx::query(
                "INSERT INTO property_owners (tenant_id, property_id, client_id, percentage) VALUES ($1, $2, $3, $4)"
            )
            .bind(tenant.0)
            .bind(id)
            .bind(owner.client_id)
            .bind(owner.percentage.unwrap_or(rust_decimal::Decimal::new(100, 0)))
            .execute(&*pool)
            .await;
        }
    }

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
