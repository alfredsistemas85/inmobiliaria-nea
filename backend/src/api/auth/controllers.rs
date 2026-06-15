use crate::{
    api::auth::dtos::{
        AuthResponse, ChangePasswordRequest, LoginRequest, MeResponse, RefreshRequest, VerifyEmailRequest,
    },
    core::security::{
        jwt::{generate_jwt, generate_refresh_jwt, verify_refresh_jwt, Claims},
        masking::mask_email,
        password::{hash_password, verify_password},
    },
    infrastructure::database::{users::UserRepository, tenants::TenantRepository},
};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    Extension,
};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub async fn login(
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    let superadmin_email = std::env::var("SUPERADMIN_EMAIL").unwrap_or_else(|_| "agentech.nea@gmail.com".to_string());
    
    if payload.identifier == superadmin_email {
        let superadmin_password = std::env::var("SUPERADMIN_PASSWORD").unwrap_or_else(|_| "xEnEizE@41".to_string());
        if payload.password == superadmin_password {
            let access_token = generate_jwt(Uuid::nil(), None, "super_admin", true)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            let refresh_token = generate_refresh_jwt(Uuid::nil(), None, "super_admin", true)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                
            tracing::info!("LOGIN_SUCCESS: super_admin hardcoded");
            return Ok(Json(AuthResponse {
                access_token,
                refresh_token,
                user_id: Uuid::nil(),
                tenant_id: None,
                role: "super_admin".to_string(),
                first_name: Some("Super".to_string()),
                last_name: Some("Admin".to_string()),
            }));
        } else {
            tracing::warn!("LOGIN_FAILED: invalid credentials for superadmin");
            return Err(StatusCode::UNAUTHORIZED);
        }
    }

    let repo = UserRepository::new(pool.clone());
    
    let user_data = if payload.identifier.contains('@') {
        repo.find_with_role_by_email(&payload.identifier)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    } else {
        let tenant_repo = TenantRepository::new(pool.clone());
        let tenant = tenant_repo
            .find_by_cuit(&payload.identifier)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
        if let Some(tenant) = tenant {
            let row = sqlx::query(
                r#"SELECT u.id, u.tenant_id, u.role_id, u.email, u.password_hash, u.first_name, u.last_name, u.is_active, u.email_verified_at, u.verification_token, u.verification_sent_at, u.email_type, u.created_at, u.updated_at, r.name as role_name
                   FROM users u LEFT JOIN roles r ON u.role_id = r.id 
                   WHERE u.tenant_id = $1 AND r.name = 'tenant_admin' AND u.deleted_at IS NULL LIMIT 1"#
            )
            .bind(tenant.id)
            .fetch_optional(&*pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
            if let Some(r) = row {
                use sqlx::Row;
                let user = crate::models::user::User {
                    id: r.try_get("id").unwrap(),
                    tenant_id: r.try_get("tenant_id").unwrap(),
                    role_id: r.try_get("role_id").unwrap(),
                    email: r.try_get("email").unwrap(),
                    password_hash: r.try_get("password_hash").unwrap(),
                    first_name: r.try_get("first_name").unwrap(),
                    last_name: r.try_get("last_name").unwrap(),
                    is_active: r.try_get("is_active").unwrap(),
                    email_verified_at: r.try_get("email_verified_at").unwrap(),
                    verification_token: r.try_get("verification_token").unwrap(),
                    verification_sent_at: r.try_get("verification_sent_at").unwrap(),
                    email_type: r.try_get("email_type").unwrap(),
                    created_at: r.try_get("created_at").unwrap(),
                    updated_at: r.try_get("updated_at").unwrap(),
                };
                let role_name: String = r.try_get("role_name").unwrap_or_else(|_| "tenant_admin".to_string());
                Some((user, role_name))
            } else {
                None
            }
        } else {
            None
        }
    };

    if let Some((user, role)) = user_data {
        if verify_password(&payload.password, &user.password_hash).unwrap_or(false) {
            if user.email_verified_at.is_none() {
                tracing::warn!(
                    "LOGIN_BLOCKED_UNVERIFIED: user_id={} email={}",
                    user.id,
                    mask_email(&user.email)
                );
                return Err(StatusCode::FORBIDDEN);
            }

            if let Some(tenant_id) = user.tenant_id {
                let tenant_repo = TenantRepository::new(pool.clone());
                if let Ok(Some(tenant)) = tenant_repo.find_by_id(tenant_id).await {
                    if tenant.status.as_deref() != Some("ACTIVE") || tenant.is_active != Some(true) {
                        tracing::warn!("LOGIN_BLOCKED_INACTIVE_TENANT: tenant_id={:?}", tenant_id);
                        return Err(StatusCode::FORBIDDEN);
                    }
                } else {
                    return Err(StatusCode::UNAUTHORIZED);
                }
            }

            let email_verified = true;
            let access_token = generate_jwt(user.id, user.tenant_id, &role, email_verified)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            let refresh_token = generate_refresh_jwt(user.id, user.tenant_id, &role, email_verified)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            tracing::info!(
                "LOGIN_SUCCESS: user_id={} tenant_id={:?} identifier={} email={}",
                user.id,
                user.tenant_id,
                payload.identifier,
                mask_email(&user.email)
            );

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
    tracing::warn!(
        "LOGIN_FAILED: invalid credentials for identifier={}",
        payload.identifier
    );
    Err(StatusCode::UNAUTHORIZED)
}

pub async fn refresh(
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<RefreshRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    let claims = verify_refresh_jwt(&payload.refresh_token).map_err(|_| {
        tracing::warn!("TOKEN_REFRESH_FAILED: invalid token");
        StatusCode::UNAUTHORIZED
    })?;

    if claims.role == "super_admin" && claims.sub == Uuid::nil() {
        let access_token = generate_jwt(Uuid::nil(), None, "super_admin", true)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let new_refresh_token = generate_refresh_jwt(Uuid::nil(), None, "super_admin", true)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        return Ok(Json(AuthResponse {
            access_token,
            refresh_token: new_refresh_token,
            user_id: Uuid::nil(),
            tenant_id: None,
            role: "super_admin".to_string(),
            first_name: Some("Super".to_string()),
            last_name: Some("Admin".to_string()),
        }));
    }

    let repo = UserRepository::new(pool.clone());
    let user_data = repo
        .find_with_role_by_id(claims.sub, claims.tenant_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some((user, role)) = user_data {
        if user.email_verified_at.is_none() {
            tracing::warn!(
                "TOKEN_REFRESH_BLOCKED_UNVERIFIED: user_id={}",
                user.id
            );
            return Err(StatusCode::FORBIDDEN);
        }

        if let Some(tenant_id) = user.tenant_id {
            let tenant_repo = TenantRepository::new(pool.clone());
            if let Ok(Some(tenant)) = tenant_repo.find_by_id(tenant_id).await {
                if tenant.status.as_deref() != Some("ACTIVE") || tenant.is_active != Some(true) {
                    tracing::warn!("REFRESH_BLOCKED_INACTIVE_TENANT: tenant_id={:?}", tenant_id);
                    return Err(StatusCode::FORBIDDEN);
                }
            } else {
                return Err(StatusCode::UNAUTHORIZED);
            }
        }

        let email_verified = true;
        let access_token = generate_jwt(user.id, user.tenant_id, &role, email_verified)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let new_refresh_token = generate_refresh_jwt(user.id, user.tenant_id, &role, email_verified)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        tracing::info!(
            "TOKEN_REFRESH_SUCCESS: user_id={} tenant_id={:?}",
            user.id,
            user.tenant_id
        );

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

    tracing::warn!("TOKEN_REFRESH_FAILED: user not found after verifying token");
    Err(StatusCode::UNAUTHORIZED)
}

pub async fn me(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<MeResponse>, StatusCode> {
    if claims.role == "super_admin" && claims.sub == Uuid::nil() {
        let superadmin_email = std::env::var("SUPERADMIN_EMAIL").unwrap_or_else(|_| "agentech.nea@gmail.com".to_string());
        return Ok(Json(MeResponse {
            id: Uuid::nil(),
            email: superadmin_email,
            first_name: Some("Super".to_string()),
            last_name: Some("Admin".to_string()),
            role: "super_admin".to_string(),
            tenant_id: None,
        }));
    }

    let repo = UserRepository::new(pool);
    let user_data = repo
        .find_with_role_by_id(claims.sub, claims.tenant_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

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
    tracing::info!("LOGOUT: logout requested");
    Ok(StatusCode::OK)
}

pub async fn change_password(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<ChangePasswordRequest>,
) -> Result<StatusCode, StatusCode> {
    if claims.role == "super_admin" && claims.sub == Uuid::nil() {
        // Superadmin password is in env var, cannot be changed here
        return Err(StatusCode::FORBIDDEN);
    }

    let repo = UserRepository::new(pool.clone());
    let user = repo
        .find_by_id(claims.sub, claims.tenant_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(user) = user {
        if verify_password(&payload.current_password, &user.password_hash).unwrap_or(false) {
            let new_hash = hash_password(&payload.new_password)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            sqlx::query("UPDATE users SET password_hash = $1 WHERE id = $2")
                .bind(new_hash)
                .bind(user.id)
                .execute(&*pool)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            tracing::info!(
                "PASSWORD_CHANGED: user_id={} tenant_id={:?}",
                user.id,
                user.tenant_id
            );
            return Ok(StatusCode::OK);
        }
    }
    Err(StatusCode::UNAUTHORIZED)
}

pub async fn verify_email(
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<VerifyEmailRequest>,
) -> Result<StatusCode, StatusCode> {
    let repo = UserRepository::new(pool.clone());
    let user = repo
        .find_by_verification_token(&payload.token)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(u) = user {
        repo.update_email_verification(u.id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
        if let Some(tid) = u.tenant_id {
            let tenant_repo = TenantRepository::new(pool.clone());
            tenant_repo.update_status(tid, "ACTIVE").await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            tracing::info!("TENANT_ACTIVATED: tenant_id={:?}", tid);
        }
        
        tracing::info!("EMAIL_VERIFIED: user_id={}", u.id);
        return Ok(StatusCode::OK);
    }
    
    tracing::warn!("EMAIL_VERIFICATION_FAILED: invalid or missing token");
    Err(StatusCode::BAD_REQUEST)
}
