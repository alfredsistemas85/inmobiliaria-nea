use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateClientDto {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: String,
    pub email: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateClientDto {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub notes: Option<String>,
}
