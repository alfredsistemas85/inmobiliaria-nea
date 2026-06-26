use crate::core::security::jwt::Claims;
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub enum AppError {
    InternalServerError,
    NotFoundError,
    Unauthorized,
    BadRequest(String),
}

pub async fn error_logging_middleware(
    State(pool): State<Arc<PgPool>>,
    req: Request,
    next: Next,
) -> Response {
    let method = req.method().to_string();
    let uri = req.uri().to_string();

    // Extract tenant_id if available in claims
    let mut tenant_id: Option<Uuid> = None;
    if let Some(claims) = req.extensions().get::<Claims>() {
        tenant_id = claims.tenant_id;
    }

    let response = next.run(req).await;
    let status = response.status();

    if status.is_server_error() {
        tracing::error!("500 Interceptado en middleware: {} {}", method, uri);

        // We could extract the body if needed, but since it's already consumed,
        // it's tricky without a custom body wrapper.
        // We will just log the occurrence.

        let error_msg = format!("HTTP {} - Server Error", status.as_u16());

        let pool_clone = pool.clone();
        tokio::spawn(async move {
            let _ = sqlx::query(
                "INSERT INTO system_errors (id, tenant_id, error_type, endpoint, method, error_message)
                 VALUES ($1, $2, $3, $4, $5, $6)"
            )
            .bind(Uuid::new_v4())
            .bind(tenant_id)
            .bind("500_INTERNAL_ERROR")
            .bind(uri)
            .bind(method)
            .bind(error_msg)
            .execute(&*pool_clone)
            .await;
        });
    }

    response
}
