use axum::{
    extract::{State, Json},
    http::StatusCode,
    response::IntoResponse,
    Extension,
};
use sqlx::PgPool;
use std::sync::Arc;
use crate::{
    api::auth::dtos::{LoginRequest, AuthResponse, RefreshRequest, ChangePasswordRequest},
    core::security::{jwt::{generate_jwt, verify_jwt, Claims}, password::{verify_password, hash_password}},
    infrastructure::database::users::UserRepository,
};

pub async fn login(
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    let repo = UserRepository::new(pool);
    let user = repo.find_by_email(&payload.email).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(user) = user {
        if verify_password(&payload.password, &user.password_hash).unwrap_or(false) {
            let role = "tenant_admin"; // Ideally, fetch from DB
            let access_token = generate_jwt(user.id, user.tenant_id, role).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            let refresh_token = "dummy_refresh_token".to_string(); // In production, generate securely and store hash
            
            return Ok(Json(AuthResponse { access_token, refresh_token }));
        }
    }
    
    Err(StatusCode::UNAUTHORIZED)
}

pub async fn refresh(
    Json(_payload): Json<RefreshRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn logout() -> Result<StatusCode, StatusCode> {
    Ok(StatusCode::OK)
}

pub async fn change_password(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<ChangePasswordRequest>,
) -> Result<StatusCode, StatusCode> {
    let repo = UserRepository::new(pool.clone());
    let user = repo.find_by_id(claims.sub, claims.tenant_id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(user) = user {
        if verify_password(&payload.current_password, &user.password_hash).unwrap_or(false) {
            let new_hash = hash_password(&payload.new_password).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
            sqlx::query("UPDATE users SET password_hash = $1 WHERE id = $2")
                .bind(new_hash)
                .bind(user.id)
                .execute(&*pool)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            return Ok(StatusCode::OK);
        }
    }
    Err(StatusCode::UNAUTHORIZED)
}
