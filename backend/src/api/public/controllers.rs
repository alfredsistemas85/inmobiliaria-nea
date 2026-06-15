use crate::{
    api::public::{
        dtos::{
            BootstrapResponse, PortalConfig, PublicAgentInfo, PublicPropertyDetailResponse,
            PublicPropertyFilter, PublicPropertyResponse, TenantInfo,
        },
        routes::PublicState,
    },
    infrastructure::database::{
        public_properties::PublicPropertyRepository, tenants::TenantRepository,
    },
    models::common::PaginatedResponse,
};
use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use redis::AsyncCommands;
use std::sync::Arc;
use uuid::Uuid;

pub async fn bootstrap(
    State(state): State<PublicState>,
    headers: HeaderMap,
) -> Result<Json<BootstrapResponse>, StatusCode> {
    let host = headers
        .get("x-forwarded-host")
        .and_then(|h| h.to_str().ok())
        .or_else(|| headers.get("host").and_then(|h| h.to_str().ok()))
        .unwrap_or("localhost");

    let slug = extract_slug(host);

    let repo = TenantRepository::new(state.pool);
    let tenant = repo
        .find_by_slug(&slug)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(BootstrapResponse {
        tenant: TenantInfo {
            id: tenant.id,
            name: tenant.business_name,
            slug: tenant.slug.unwrap_or_default(),
            logo_url: None,
            phone: tenant.phone.clone(),
            email: None,
            whatsapp: tenant.phone,
            address: tenant.address,
        },
        portal: PortalConfig {
            allow_contact_form: true,
            allow_whatsapp: true,
        },
    }))
}

pub async fn get_properties(
    State(state): State<PublicState>,
    Query(filter): Query<PublicPropertyFilter>,
) -> Result<Json<PaginatedResponse<PublicPropertyResponse>>, StatusCode> {
    let repo = PublicPropertyRepository::new(state.pool);

    let limit = filter.limit.unwrap_or(20);
    let offset = filter.offset.unwrap_or(0);

    let paginated = repo
        .list(
            filter.tenant_id,
            limit,
            offset,
            filter.operation_type.as_deref(),
            filter.property_type.as_deref(),
            filter.city.as_deref(),
            filter.price_min,
            filter.price_max,
            filter.bedrooms,
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut responses = Vec::new();
    for prop in paginated.data {
        let main_image = repo
            .get_main_image(prop.id, filter.tenant_id)
            .await
            .unwrap_or(None);

        responses.push(PublicPropertyResponse {
            id: prop.id,
            title: prop.title,
            description: prop.description,
            property_type: prop.property_type,
            operation_type: prop.operation_type,
            price: prop.price,
            currency: prop.currency,
            address: prop.address,
            city: prop.city,
            province: prop.province,
            square_meters: prop.square_meters,
            bedrooms: prop.bedrooms,
            bathrooms: prop.bathrooms,
            features: prop.features,
            main_image_url: main_image,
            created_at: prop.created_at,
        });
    }

    Ok(Json(PaginatedResponse {
        data: responses,
        total: paginated.total,
        limit: paginated.limit,
        offset: paginated.offset,
    }))
}

pub async fn get_property(
    State(state): State<PublicState>,
    Path(id): Path<Uuid>,
    Query(filter): Query<PublicPropertyFilter>,
    headers: HeaderMap,
) -> Result<Json<PublicPropertyDetailResponse>, StatusCode> {
    let tenant_id = filter.tenant_id;
    let repo = PublicPropertyRepository::new(state.pool.clone());

    // Validate existence via list (lazy approach to reuse code or we can add a specific get in public.rs)
    // Actually, we don't have a specific find_by_id in public repo yet. Let's borrow from PropertyRepository.
    let property_repo =
        crate::infrastructure::database::properties::PropertyRepository::new(state.pool.clone());
    let prop = property_repo
        .find_by_id(id, tenant_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Only allow DISPONIBLE or let's assume public only shows if not deleted.
    if prop.status.as_deref() != Some("DISPONIBLE") {
        return Err(StatusCode::NOT_FOUND);
    }

    let images = repo.get_all_images(id, tenant_id).await.unwrap_or_default();
    let documents = repo.get_documents(id, tenant_id).await.unwrap_or_default();

    // Tracker views in Redis
    let mut redis_conn = state
        .redis
        .get_multiplexed_async_connection()
        .await
        .unwrap();
    let ip = headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .or_else(|| headers.get("x-real-ip").and_then(|v| v.to_str().ok()))
        .unwrap_or("unknown_ip");

    let visitor_key = format!("property:visitor:{}:{}", id, ip);
    let is_new_visitor: i64 = redis_conn.exists(&visitor_key).await.unwrap_or(0);

    if is_new_visitor == 0 {
        let view_key = format!("property:views:{}", id);
        let _: () = redis_conn.incr(&view_key, 1).await.unwrap_or(());
        let _: () = redis_conn
            .set_ex(&visitor_key, "1", 86400)
            .await
            .unwrap_or(());
    }

    Ok(Json(PublicPropertyDetailResponse {
        id: prop.id,
        title: prop.title,
        description: prop.description,
        property_type: prop.property_type,
        operation_type: prop.operation_type,
        price: prop.price,
        currency: prop.currency,
        address: prop.address,
        city: prop.city,
        province: prop.province,
        square_meters: prop.square_meters,
        bedrooms: prop.bedrooms,
        bathrooms: prop.bathrooms,
        features: prop.features,
        images,
        documents,
        agent: None, // Agent relationship not stored directly in property. To be extended if needed.
    }))
}

pub async fn get_featured(
    State(state): State<PublicState>,
    Query(filter): Query<PublicPropertyFilter>,
) -> Result<Json<Vec<PublicPropertyResponse>>, StatusCode> {
    let repo = PublicPropertyRepository::new(state.pool);
    let limit = filter.limit.unwrap_or(6);

    let featured = repo
        .get_featured(filter.tenant_id, limit)
        .await
        .unwrap_or_default();

    let mut responses = Vec::new();
    for prop in featured {
        let main_image = repo
            .get_main_image(prop.id, filter.tenant_id)
            .await
            .unwrap_or(None);
        responses.push(PublicPropertyResponse {
            id: prop.id,
            title: prop.title,
            description: prop.description,
            property_type: prop.property_type,
            operation_type: prop.operation_type,
            price: prop.price,
            currency: prop.currency,
            address: prop.address,
            city: prop.city,
            province: prop.province,
            square_meters: prop.square_meters,
            bedrooms: prop.bedrooms,
            bathrooms: prop.bathrooms,
            features: prop.features,
            main_image_url: main_image,
            created_at: prop.created_at,
        });
    }

    Ok(Json(responses))
}

pub async fn create_public_lead(
    State(state): State<PublicState>,
    headers: HeaderMap,
    Json(payload): Json<crate::api::public::dtos::CreatePublicLeadDto>,
) -> Result<StatusCode, StatusCode> {
    // 1. Honeypot check
    if let Some(website) = &payload.website {
        if !website.is_empty() {
            tracing::warn!("Honeypot triggered for tenant {}", payload.tenant_id);
            return Ok(StatusCode::OK); // Fake success for bots
        }
    }

    // 2. IP extraction & Rate Limit
    let ip = headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .or_else(|| headers.get("x-real-ip").and_then(|v| v.to_str().ok()))
        .unwrap_or("unknown_ip");

    let rate_limit_key = format!("rate_limit:ip:{}", ip);
    let mut redis_conn = state
        .redis
        .get_multiplexed_async_connection()
        .await
        .unwrap();

    let current_requests: i64 = redis_conn.get(&rate_limit_key).await.unwrap_or(0);
    if current_requests >= 3 {
        tracing::warn!("Rate limit exceeded for IP {}", ip);
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }

    let _: () = redis_conn.incr(&rate_limit_key, 1).await.unwrap_or(());
    if current_requests == 0 {
        let _: () = redis_conn.expire(&rate_limit_key, 3600).await.unwrap_or(());
        // 1 hour TTL
    }

    // 3. Split Name into First/Last
    let mut parts = payload.name.split_whitespace();
    let first_name = parts.next().unwrap_or("").to_string();
    let last_name = parts.collect::<Vec<&str>>().join(" ");

    // 4. Create Client
    let client_repo =
        crate::infrastructure::database::clients::ClientRepository::new(state.pool.clone());
    let phone_str = payload.phone.unwrap_or_default();

    let client = client_repo
        .create(
            payload.tenant_id,
            Some(&first_name),
            if last_name.is_empty() {
                None
            } else {
                Some(&last_name)
            },
            &phone_str,
            payload.email.as_deref(),
            Some("Cliente ingresado vía Portal Público Web"),
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 5. Create Lead
    let lead_repo = crate::infrastructure::database::leads::LeadRepository::new(state.pool.clone());
    let lead = lead_repo
        .create(
            payload.tenant_id,
            client.id,
            payload.property_id,
            Some("NUEVO"),
            Some("WEB"),
            None, // No agent assigned initially
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 6. Log Activity
    let details = format!(
        "ORIGIN = PUBLIC_PORTAL\nNombre: {}\nMensaje: {}",
        payload.name,
        payload.message.unwrap_or_default()
    );
    let _ = lead_repo
        .log_activity(
            payload.tenant_id,
            lead.id,
            None, // No user_id
            "PublicWebInquiry",
            Some(&details),
        )
        .await;

    Ok(StatusCode::CREATED)
}

fn extract_slug(host: &str) -> String {
    let host_no_port = host.split(':').next().unwrap_or(host);
    let parts: Vec<&str> = host_no_port.split('.').collect();

    if host_no_port == "localhost" {
        "inmonea".to_string()
    } else if parts.len() >= 3 {
        parts[0].to_string()
    } else {
        parts[0].to_string()
    }
}
