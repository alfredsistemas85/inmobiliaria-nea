use axum::{
    extract::{State, Json},
    http::StatusCode,
    response::IntoResponse,
    Extension,
};
use sqlx::PgPool;
use std::sync::Arc;
use crate::{
    api::auth::dtos::{LoginRequest, AuthResponse, RefreshRequest, ChangePasswordRequest, MeResponse},
    core::security::{jwt::{generate_jwt, generate_refresh_jwt, verify_refresh_jwt, Claims}, password::{verify_password, hash_password}},
    infrastructure::database::users::UserRepository,
};

pub async fn login(
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    let repo = UserRepository::new(pool);
    let user_data = repo.find_with_role_by_email(&payload.email).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some((user, role)) = user_data {
        if verify_password(&payload.password, &user.password_hash).unwrap_or(false) {
            let access_token = generate_jwt(user.id, user.tenant_id, &role).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            let refresh_token = generate_refresh_jwt(user.id, user.tenant_id, &role).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
            return Ok(Json(AuthResponse { 
                access_token, 
                refresh_token,
                user_id: user.id,
                tenant_id: user.tenant_id,
                role,
                first_name: user.first_name,
                last_name: user.last_name,
            }));
        }
    }
    
    Err(StatusCode::UNAUTHORIZED)
}

pub async fn refresh(
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<RefreshRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    let claims = verify_refresh_jwt(&payload.refresh_token).map_err(|_| StatusCode::UNAUTHORIZED)?;
    
    let repo = UserRepository::new(pool);
    let user_data = repo.find_with_role_by_id(claims.sub, claims.tenant_id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some((user, role)) = user_data {
        let access_token = generate_jwt(user.id, user.tenant_id, &role).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let new_refresh_token = generate_refresh_jwt(user.id, user.tenant_id, &role).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        return Ok(Json(AuthResponse { 
            access_token, 
            refresh_token: new_refresh_token,
            user_id: user.id,
            tenant_id: user.tenant_id,
            role,
            first_name: user.first_name,
            last_name: user.last_name,
        }));
    }

    Err(StatusCode::UNAUTHORIZED)
}

pub async fn me(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<MeResponse>, StatusCode> {
    let repo = UserRepository::new(pool);
    let user_data = repo.find_with_role_by_id(claims.sub, claims.tenant_id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some((user, role)) = user_data {
        return Ok(Json(MeResponse {
            id: user.id,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            role,
            tenant_id: user.tenant_id,
        }));
    }

    Err(StatusCode::UNAUTHORIZED)
}

pub async fn logout() -> Result<StatusCode, StatusCode> {
    // With stateless JWTs, logout is purely a frontend operation (deleting tokens).
    // In the future, this could blacklist tokens in Redis.
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
