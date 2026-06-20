use crate::{
    api::users::dtos::{CreateUserDto, UpdateUserDto, UserResponseDto},
    core::security::jwt::Claims,
    core::security::password::hash_password,
    infrastructure::database::users::UserRepository,
    models::user::User,
};
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    Extension,
};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub async fn list_users(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<UserResponseDto>>, StatusCode> {
    let users = if let Some(tenant_id) = claims.tenant_id {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE tenant_id = $1 AND deleted_at IS NULL")
            .bind(tenant_id)
            .fetch_all(&*pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    } else {
        if claims.role != "super_admin" {
            return Err(StatusCode::FORBIDDEN);
        }
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE deleted_at IS NULL")
            .fetch_all(&*pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    };

    Ok(Json(users.into_iter().map(UserResponseDto::from).collect()))
}

pub async fn get_user(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<UserResponseDto>, StatusCode> {
    let repo = UserRepository::new(pool);
    let user = repo
        .find_by_id(id, claims.tenant_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match user {
        Some(u) => Ok(Json(UserResponseDto::from(u))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn create_user(
    State(pool): State<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateUserDto>,
) -> Result<Json<UserResponseDto>, StatusCode> {
    let hashed_pw =
        hash_password(&payload.password).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if payload.role == crate::models::role::UserRole::Superadmin {
        tracing::warn!("SECURITY_VIOLATION: Attempt to assign super_admin role on creation");
        return Err(StatusCode::FORBIDDEN);
    }

    // Safety check: a non-super_admin MUST have a tenant_id to create a user
    let target_tenant_id = claims.tenant_id.or_else(|| {
        if claims.role == "super_admin" || claims.role == "SUPERADMIN" {
            None
        } else {
            Some(Uuid::nil())
        }
    });
    if claims.tenant_id.is_none() && claims.role != "super_admin" && claims.role != "SUPERADMIN" {
        return Err(StatusCode::FORBIDDEN);
    }

    if crate::core::utils::email_validator::is_disposable(&payload.email) {
        return Err(StatusCode::BAD_REQUEST);
    }

    let email_type = crate::core::utils::email_validator::get_email_type(&payload.email);
    let v_token = Uuid::new_v4().to_string();

    let user = User {
        id: Uuid::new_v4(),
        tenant_id: target_tenant_id,
        role: Some(payload.role),
        email: payload.email,
        password_hash: hashed_pw,
        first_name: payload.first_name,
        last_name: payload.last_name,
        is_active: Some(true),
        email_verified_at: None,
        verification_token: Some(v_token.clone()),
        verification_sent_at: Some(chrono::Utc::now()),
        email_type: Some(email_type),
        onboarding_token: None,
        onboarding_token_expires_at: None,
        created_at: None,
        updated_at: None,
    };

    let repo = UserRepository::new(pool);
    let created = repo
        .create(user)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    tracing::info!("EMAIL_SENT: to={} token={}", created.email, v_token);

    let frontend_url = std::env::var("FRONTEND_URL").unwrap_or_else(|_| "https://inmonea.agentech.ar".to_string());
    let verify_url = format!("{}/verify-email?token={}", frontend_url, v_token);
    let body = format!(
        "Bienvenido a Inmobiliarias NEA.\n\nPor favor, verifica tu correo y activa tu cuenta haciendo clic en el siguiente enlace:\n{}",
        verify_url
    );
    
    let _ = crate::core::utils::email_sender::send_email(&created.email, "Verifica tu cuenta - Inmobiliarias NEA", &body).await;

    Ok(Json(UserResponseDto::from(created)))
}

pub async fn update_user(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<UpdateUserDto>,
) -> Result<Json<UserResponseDto>, StatusCode> {
    let repo = UserRepository::new(pool.clone());
    let mut user = repo
        .find_by_id(id, claims.tenant_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Only allow updating users within the same tenant
    if claims.tenant_id.is_some() && user.tenant_id != claims.tenant_id {
        return Err(StatusCode::FORBIDDEN);
    }
    if claims.tenant_id.is_none() && claims.role != "super_admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    if let Some(email) = payload.email {
        user.email = email;
    }
    if let Some(first_name) = payload.first_name {
        user.first_name = Some(first_name);
    }
    if let Some(last_name) = payload.last_name {
        user.last_name = Some(last_name);
    }
    if let Some(role) = payload.role {
        if role == crate::models::role::UserRole::Superadmin {
            tracing::warn!("SECURITY_VIOLATION: Attempt to assign superadmin role");
            return Err(StatusCode::FORBIDDEN);
        }
        user.role = Some(role);
    }
    if let Some(is_active) = payload.is_active {
        user.is_active = Some(is_active);
    }

    let updated = sqlx::query_as::<_, User>(
        r#"UPDATE users SET email = $1, first_name = $2, last_name = $3, role = $4, is_active = $5 
           WHERE id = $6 RETURNING *"#
    )
    .bind(&user.email)
    .bind(&user.first_name)
    .bind(&user.last_name)
    .bind(&user.role)
    .bind(&user.is_active)
    .bind(user.id)
    .fetch_one(&*pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(UserResponseDto::from(updated)))
}

pub async fn delete_user(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> Result<StatusCode, StatusCode> {
    let rows_affected = if let Some(tenant_id) = claims.tenant_id {
        sqlx::query(
            "UPDATE users SET deleted_at = CURRENT_TIMESTAMP WHERE id = $1 AND tenant_id = $2",
        )
        .bind(id)
        .bind(tenant_id)
        .execute(&*pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .rows_affected()
    } else {
        if claims.role != "super_admin" {
            return Err(StatusCode::FORBIDDEN);
        }
        sqlx::query("UPDATE users SET deleted_at = CURRENT_TIMESTAMP WHERE id = $1")
            .bind(id)
            .execute(&*pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .rows_affected()
    };

    if rows_affected > 0 {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
