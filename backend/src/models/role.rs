use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema, sqlx::Type, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "user_role", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserRole {
    Superadmin,
    AdminInmobiliaria,
    Supervisor,
    Agente,
    Operador,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, sqlx::FromRow, Clone)]
pub struct Role {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, sqlx::FromRow, Clone)]
pub struct Permission {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}
