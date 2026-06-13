use axum::{routing::get, Router};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod api;
mod core;
mod infrastructure;
mod models;
mod config;

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
            models::lead::LeadStatus,
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

#[utoipa::path(
    get,
    path = "/api/health",
    responses(
        (status = 200, description = "Service is healthy")
    )
)]
async fn health_check() -> &'static str {
    "OK"
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
    let rate_limit_state = Arc::new(core::security::rate_limit::RateLimitState { redis_client });

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/api/health", get(health_check))
        .nest("/api/auth", api::auth::router(shared_pool.clone()))
        .nest("/api/users", api::users::router(shared_pool.clone()))
        .nest("/api/properties", api::properties::router(shared_pool.clone()))
        .nest("/api/tenants", api::tenants::router(shared_pool.clone()))
        .nest("/api/roles", api::roles::router(shared_pool.clone()))
        .nest("/api/clients", api::clients::routes::router(shared_pool.clone()))
        .nest("/api/appointments", api::appointments::routes::router(shared_pool.clone()))
        // Rate limiting for the entire API except swagger
        .layer(axum::middleware::from_fn_with_state(
            rate_limit_state.clone(),
            core::security::rate_limit::rate_limit_middleware,
        ));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Servidor escuchando en http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
