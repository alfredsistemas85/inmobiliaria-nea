use axum::{
    extract::Request,
    http::StatusCode,
    middleware::{self, Next},
    response::Response,
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use std::sync::Arc;

use crate::api::whatsapp::controllers::{
    assign_conversation, close_conversation, create_instance, get_instance_status, get_qr,
    list_conversations, list_messages, logout_instance, reopen_conversation, run_reminders,
    send_chat_message, take_conversation, unassign_conversation, webhook,
};
use crate::core::rbac::middleware::require_tenant_admin;
use crate::core::tenant::middleware::tenant_middleware;

pub fn router(pool: Arc<PgPool>) -> Router {
    let agent_routes = Router::new()
        .route("/conversations", get(list_conversations))
        .route(
            "/conversations/:id/messages",
            get(list_messages).post(send_chat_message),
        )
        .route("/conversations/:id/take", post(take_conversation))
        .route_layer(middleware::from_fn(tenant_middleware))
        .with_state(pool.clone());

    let admin_routes = Router::new()
        .route("/conversations/:id/assign", post(assign_conversation))
        .route("/conversations/:id/unassign", post(unassign_conversation))
        .route("/conversations/:id/close", post(close_conversation))
        .route("/conversations/:id/reopen", post(reopen_conversation))
        .route("/instance", get(get_instance_status).post(create_instance))
        .route("/instance/status", get(get_instance_status))
        .route("/instance/qr", get(get_qr))
        .route("/instance/connect", post(get_qr))
        .route("/instance/logout", post(logout_instance))
        .route("/instance/disconnect", post(logout_instance))
        .route_layer(middleware::from_fn(require_tenant_admin))
        .route_layer(middleware::from_fn(tenant_middleware))
        .with_state(pool.clone());

    let webhook_routes = Router::new()
        .route("/webhook/:tenant_id", post(webhook))
        .route("/reminders/run", post(run_reminders))
        .route_layer(middleware::from_fn(require_webhook_secret))
        .with_state(pool);

    Router::new()
        .merge(agent_routes)
        .merge(admin_routes)
        .merge(webhook_routes)
}

async fn require_webhook_secret(
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let expected_secret = std::env::var("EVOLUTION_API_KEY").unwrap_or_else(|_| "apikey_evolution".to_string());
    if let Some(auth_header) = req.headers().get("apikey") {
        if auth_header.to_str().unwrap_or_default() == expected_secret {
            return Ok(next.run(req).await);
        }
    }
    Err(StatusCode::UNAUTHORIZED)
}
