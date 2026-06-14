use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateLeadDto {
    pub client_id: Uuid,
    pub property_id: Option<Uuid>,
    pub status: Option<String>,
    pub source: Option<String>,
    pub assigned_to: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateLeadDto {
    pub status: Option<String>,
    pub source: Option<String>,
    pub assigned_to: Option<Uuid>,
}
