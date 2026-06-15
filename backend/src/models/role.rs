use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

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
