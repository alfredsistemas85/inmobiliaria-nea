use axum::{extract::Request, middleware::Next, response::Response};
use axum::{routing::get, Router};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::{cors::CorsLayer, services::ServeDir};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use backend::{api, config, core, infrastructure, models};

#[derive(OpenApi)]
#[openapi(
    paths(health_check),
    components(
        schemas(
            models::tenant::Tenant,
            models::user::User,
            models::role::Role,
            models::role::Permission,
            models::property::Property,
            models::property::PropertyImage,
            models::property::PropertyDocument,
            api::auth::dtos::LoginRequest,
            api::auth::dtos::AuthResponse,
            api::auth::dtos::RefreshRequest,
            api::auth::dtos::ChangePasswordRequest,
            api::users::dtos::CreateUserDto,
            api::users::dtos::UpdateUserDto,
            api::users::dtos::UserResponseDto,
            api::properties::dtos::CreatePropertyDto,
            api::properties::dtos::PropertyResponseDto,
            api::tenants::dtos::CreateTenantDto,
            api::tenants::dtos::TenantResponseDto,
            api::roles::dtos::RoleResponseDto,
        )
    ),
    tags(
        (name = "health", description = "Health checks API")
    )
)]
struct ApiDoc;

#[derive(Clone)]
pub struct HealthState {
    pub pool: Arc<PgPool>,
    pub redis_client: Arc<redis::Client>,
}

#[derive(serde::Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String,
    pub checks: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_time_ms: Option<u128>,
}

#[utoipa::path(
    get,
    path = "/api/health",
    responses(
        (status = 200, description = "Service is healthy")
    )
)]
async fn health_check(
    axum::extract::State(state): axum::extract::State<HealthState>,
) -> axum::Json<HealthResponse> {
    let start = std::time::Instant::now();
    let mut status = "healthy".to_string();
    let mut pg_status = "ok".to_string();
    let mut redis_status = "ok".to_string();

    if let Err(_) = sqlx::query("SELECT 1").execute(&*state.pool).await {
        pg_status = "error".to_string();
        status = "degraded".to_string();
    }

    if let Ok(mut con) = state.redis_client.get_multiplexed_async_connection().await {
        let ping: Result<String, _> = redis::cmd("PING").query_async(&mut con).await;
        if ping.is_err() {
            redis_status = "error".to_string();
            status = "degraded".to_string();
        }
    } else {
        redis_status = "error".to_string();
        status = "degraded".to_string();
    }

    let checks = serde_json::json!({
        "postgres": pg_status,
        "redis": redis_status,
    });

    axum::Json(HealthResponse {
        status,
        timestamp: chrono::Utc::now().to_rfc3339(),
        checks,
        response_time_ms: Some(start.elapsed().as_millis()),
    })
}

async fn security_headers_middleware(req: Request, next: Next) -> Response {
    let mut response = next.run(req).await;
    let headers = response.headers_mut();
    headers.insert(
        "x-frame-options",
        axum::http::HeaderValue::from_static("DENY"),
    );
    headers.insert(
        "x-content-type-options",
        axum::http::HeaderValue::from_static("nosniff"),
    );
    headers.insert(
        "referrer-policy",
        axum::http::HeaderValue::from_static("strict-origin-when-cross-origin"),
    );
    headers.insert(
        "permissions-policy",
        axum::http::HeaderValue::from_static("camera=(), microphone=(), geolocation=()"),
    );
    response
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    tracing::info!("Iniciando SaaS Inmobiliarias NEA Backend...");

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL debe estar configurada");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Error al conectar a PostgreSQL");

    let shared_pool = Arc::new(pool);
    tracing::info!("Conectado a PostgreSQL Exitosamente.");

    // Configurando Rate Limiting
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
    let redis_client = redis::Client::open(redis_url).expect("URL de Redis inválida");
    let rate_limit_state = Arc::new(core::security::rate_limit::RateLimitState {
        redis_client: redis_client.clone(),
    });

    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::AllowOrigin::predicate(
            |origin: &axum::http::HeaderValue, _request_parts: &axum::http::request::Parts| {
                if let Ok(origin_str) = origin.to_str() {
                    origin_str.ends_with(".agentech.ar")
                        || origin_str == "https://inmonea.agentech.ar"
                        || origin_str.starts_with("http://localhost")
                        || origin_str.starts_with("http://127.0.0.1")
                } else {
                    false
                }
            },
        ))
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);

    let health_router = Router::new()
        .route("/health", get(health_check))
        .with_state(HealthState {
            pool: shared_pool.clone(),
            redis_client: Arc::new(redis_client.clone()),
        });

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest("/api", health_router)
        .nest(
            "/api/auth",
            api::auth::router(shared_pool.clone(), rate_limit_state.clone()),
        )
        .nest("/api/users", api::users::router(shared_pool.clone()))
        .nest(
            "/api/properties",
            api::properties::router(shared_pool.clone(), rate_limit_state.clone()),
        )
        .nest("/api/tenants", api::tenants::router(shared_pool.clone()))
        .nest("/api/admin/system", api::system::routes::router(shared_pool.clone()))
        .nest("/api/roles", api::roles::router(shared_pool.clone()))
        .nest(
            "/api/clients",
            api::clients::routes::router(shared_pool.clone()),
        )
        .nest(
            "/api/appointments",
            api::appointments::routes::router(shared_pool.clone()),
        )
        .nest(
            "/api/leads",
            api::leads::routes::router(shared_pool.clone()),
        )
        .nest(
            "/api/dashboard",
            api::dashboard::routes::router(shared_pool.clone()),
        )
        .nest(
            "/api/whatsapp",
            api::whatsapp::routes::router(shared_pool.clone()),
        )
        .nest(
            "/api/notifications",
            api::notifications::routes::router(shared_pool.clone()),
        )
        .nest(
            "/api/reports",
            api::reports::routes::router(shared_pool.clone()),
        )
        .nest(
            "/api/public",
            api::public::routes::router(shared_pool.clone(), Arc::new(redis_client.clone())),
        )
        .nest_service("/uploads", ServeDir::new("uploads"))
        .layer(axum::middleware::from_fn(security_headers_middleware))
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Servidor escuchando en http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
