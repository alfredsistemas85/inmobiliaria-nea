use crate::core::{security::jwt::Claims, utils::email_sender::send_email};
use axum::{extract::Extension, http::StatusCode};

pub async fn email_check(Extension(claims): Extension<Claims>) -> Result<StatusCode, StatusCode> {
    if claims.role != "super_admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    let superadmin_email =
        std::env::var("SUPERADMIN_EMAIL").unwrap_or_else(|_| "agentech.nea@gmail.com".to_string());

    match send_email(
        &superadmin_email,
        "System Check: Email Provider",
        "This is an automated test from the SaaS backend to verify SMTP configuration.",
    )
    .await
    {
        Ok(_) => {
            tracing::info!("SMTP Check: Success");
            Ok(StatusCode::OK)
        }
        Err(e) => {
            tracing::error!("SMTP Check Failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
