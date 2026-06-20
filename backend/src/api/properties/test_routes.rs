use axum::{routing::{get, post}, Router, middleware};

async fn handler() {}
async fn admin_handler() {}
async fn require_admin(req: axum::extract::Request, next: middleware::Next) -> Result<axum::response::Response, axum::http::StatusCode> { Ok(next.run(req).await) }

pub fn router() -> Router {
    Router::new()
        .route("/", get(handler).post(post(admin_handler).route_layer(middleware::from_fn(require_admin))))
}
