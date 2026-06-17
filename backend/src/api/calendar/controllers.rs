use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{Redirect, IntoResponse},
    Json, Extension,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;
use std::env;
use crate::core::security::jwt::Claims;
use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl,
    Scope, TokenResponse, TokenUrl, AuthorizationCode,
};

#[derive(Serialize)]
pub struct CalendarStatusResponse {
    pub provider: Option<String>,
    pub email: Option<String>,
    pub active: bool,
}

#[derive(Deserialize)]
pub struct AuthRequest {
    pub code: String,
    pub state: String,
}

fn google_oauth_client() -> Option<BasicClient> {
    let client_id = env::var("GOOGLE_CLIENT_ID").ok()?;
    let client_secret = env::var("GOOGLE_CLIENT_SECRET").ok()?;
    let redirect_url = env::var("GOOGLE_REDIRECT_URI").unwrap_or_else(|_| "http://localhost:3000/api/calendar/google/callback".to_string());

    Some(BasicClient::new(ClientId::new(client_id))
        .set_client_secret(ClientSecret::new(client_secret))
        .set_auth_uri(AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).unwrap())
        .set_token_uri(TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).unwrap())
        .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap()))
}

pub async fn google_connect(
    Extension(claims): Extension<Claims>,
) -> Result<Redirect, StatusCode> {
    let client = google_oauth_client().ok_or(StatusCode::SERVICE_UNAVAILABLE)?;

    // We can encode the tenant_id and user_id in the state, or use a redis session.
    // For simplicity, we just pass the user token or encode it in state.
    // Wait, the redirect happens from the browser, so the user has the auth cookie. 
    // In our app, we use Bearer tokens, so clicking a link directly might not send the token.
    // Usually, the frontend fetches the `/google/connect` URL and then `window.location.href = url`.
    
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("https://www.googleapis.com/auth/calendar".to_string()))
        .add_scope(Scope::new("https://www.googleapis.com/auth/userinfo.email".to_string()))
        // state can be used to pass claims.sub to know who is connecting
        .add_extra_param("state", &claims.sub.to_string())
        .add_extra_param("access_type", "offline")
        .add_extra_param("prompt", "consent")
        .url();

    Ok(Redirect::to(auth_url.as_str()))
}

pub async fn google_callback(
    State(pool): State<Arc<PgPool>>,
    Query(query): Query<AuthRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let client = google_oauth_client().ok_or(StatusCode::SERVICE_UNAVAILABLE)?;

    // The state contains the user_id (UUID)
    let user_id = Uuid::parse_str(&query.state).map_err(|_| StatusCode::BAD_REQUEST)?;

    // Exchange the code with a token
    let token_result = client
        .exchange_code(AuthorizationCode::new(query.code))
        .request_async(oauth2::reqwest::async_http_client)
        .await
        .map_err(|e| {
            tracing::error!("OAuth exchange failed: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let access_token = token_result.access_token().secret().clone();
    let refresh_token = token_result.refresh_token().map(|t: &oauth2::RefreshToken| t.secret().clone());

    // Obtenemos el tenant_id del usuario
    let tenant_id: Uuid = sqlx::query_scalar("SELECT tenant_id FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(&*pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Upsert the token
    sqlx::query(
        r#"
        INSERT INTO calendar_integrations (tenant_id, user_id, provider, access_token, refresh_token, active)
        VALUES ($1, $2, 'google', $3, $4, true)
        ON CONFLICT (tenant_id, user_id, provider)
        DO UPDATE SET access_token = EXCLUDED.access_token, refresh_token = COALESCE(EXCLUDED.refresh_token, calendar_integrations.refresh_token), active = true, updated_at = CURRENT_TIMESTAMP
        "#
    )
    .bind(tenant_id)
    .bind(user_id)
    .bind(access_token)
    .bind(refresh_token)
    .execute(&*pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Redirect to frontend Integrations page
    // Podría ser configurable
    let frontend_url = env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:5173".to_string());
    Ok(Redirect::to(&format!("{}/dashboard/settings?integration=success", frontend_url)))
}

pub async fn get_status(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<CalendarStatusResponse>, StatusCode> {
    let row = sqlx::query(
        "SELECT provider, external_email, active FROM calendar_integrations WHERE user_id = $1 AND tenant_id = $2 AND provider = 'google'"
    )
    .bind(claims.sub)
    .bind(claims.tenant_id.unwrap())
    .fetch_optional(&*pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(record) = row {
        use sqlx::Row;
        Ok(Json(CalendarStatusResponse {
            provider: Some(record.try_get("provider").unwrap_or_default()),
            email: record.try_get("external_email").ok(),
            active: record.try_get("active").unwrap_or(false),
        }))
    } else {
        Ok(Json(CalendarStatusResponse {
            provider: None,
            email: None,
            active: false,
        }))
    }
}

pub async fn disconnect(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
) -> Result<StatusCode, StatusCode> {
    sqlx::query("DELETE FROM calendar_integrations WHERE user_id = $1 AND tenant_id = $2")
        .bind(claims.sub)
        .bind(claims.tenant_id.unwrap())
        .execute(&*pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}
