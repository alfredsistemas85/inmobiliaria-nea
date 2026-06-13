use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateUserDto {
    pub email: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub role_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateUserDto {
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub role_id: Option<Uuid>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponseDto {
    pub id: Uuid,
    pub tenant_id: Option<Uuid>,
    pub role_id: Option<Uuid>,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub is_active: Option<bool>,
}

impl From<crate::models::user::User> for UserResponseDto {
    fn from(user: crate::models::user::User) -> Self {
        Self {
            id: user.id,
            tenant_id: user.tenant_id,
            role_id: user.role_id,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            is_active: user.is_active,
        }
    }
}
