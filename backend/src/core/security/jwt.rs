use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use std::env;
use chrono::{Utc, Duration};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid,
    pub tenant_id: Option<Uuid>,
    pub role: String,
    pub exp: usize,
    pub token_type: String, // "access" or "refresh"
}

pub fn generate_jwt(user_id: Uuid, tenant_id: Option<Uuid>, role: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(15))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        tenant_id,
        role: role.to_owned(),
        exp: expiration,
        token_type: "access".to_string(),
    };

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
}

pub fn generate_refresh_jwt(user_id: Uuid, tenant_id: Option<Uuid>, role: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(7))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        tenant_id,
        role: role.to_owned(),
        exp: expiration,
        token_type: "refresh".to_string(),
    };

    let secret = env::var("JWT_REFRESH_SECRET").unwrap_or_else(|_| env::var("JWT_SECRET").expect("JWT_SECRET must be set"));
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
}

pub fn verify_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let mut validation = Validation::default();
    validation.validate_exp = true;
    let token_data = decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &validation)?;
    
    if token_data.claims.token_type != "access" {
        return Err(jsonwebtoken::errors::ErrorKind::InvalidToken.into());
    }
    
    Ok(token_data.claims)
}

pub fn verify_refresh_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_REFRESH_SECRET").unwrap_or_else(|_| env::var("JWT_SECRET").expect("JWT_SECRET must be set"));
    let mut validation = Validation::default();
    validation.validate_exp = true;
    let token_data = decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &validation)?;
    
    if token_data.claims.token_type != "refresh" {
        return Err(jsonwebtoken::errors::ErrorKind::InvalidToken.into());
    }
    
    Ok(token_data.claims)
}
