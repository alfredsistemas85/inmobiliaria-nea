use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

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
