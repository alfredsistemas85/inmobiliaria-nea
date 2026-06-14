use axum::{
    routing::{get, post},
    Router,
    middleware,
};
use sqlx::PgPool;
use std::sync::Arc;

use crate::api::whatsapp::controllers::{
    list_conversations, list_messages, send_chat_message, webhook, run_reminders,
    take_conversation, assign_conversation, unassign_conversation, close_conversation, reopen_conversation
};
use crate::core::tenant::middleware::tenant_middleware;
use crate::core::rbac::middleware::require_tenant_admin;

pub fn router(pool: Arc<PgPool>) -> Router {
    let agent_routes = Router::new()
        .route("/conversations", get(list_conversations))
        .route("/conversations/:id/messages", get(list_messages).post(send_chat_message))
        .route("/conversations/:id/take", post(take_conversation))
        .route_layer(middleware::from_fn(tenant_middleware))
        .with_state(pool.clone());

    let admin_routes = Router::new()
        .route("/conversations/:id/assign", post(assign_conversation))
        .route("/conversations/:id/unassign", post(unassign_conversation))
        .route("/conversations/:id/close", post(close_conversation))
        .route("/conversations/:id/reopen", post(reopen_conversation))
        .route_layer(middleware::from_fn(require_tenant_admin))
        .route_layer(middleware::from_fn(tenant_middleware))
        .with_state(pool.clone());

    let webhook_routes = Router::new()
        .route("/webhook/:tenant_id", post(webhook))
        .route("/reminders/run", post(run_reminders))
        .with_state(pool);

    Router::new()
        .merge(agent_routes)
        .merge(admin_routes)
        .merge(webhook_routes)
}
