use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, ToSchema)]
pub struct RoleResponseDto {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

impl From<crate::models::role::Role> for RoleResponseDto {
    fn from(role: crate::models::role::Role) -> Self {
        Self {
            id: role.id,
            name: role.name,
            description: role.description,
        }
    }
}
