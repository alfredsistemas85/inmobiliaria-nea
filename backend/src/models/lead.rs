use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, PartialEq, sqlx::Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "PascalCase")]
pub enum LeadStatus {
    Nuevo,
    Contactado,
    Interesado,
    VisitaAgendada,
    Negociacion,
    Cerrado,
    Perdido,
}

impl Default for LeadStatus {
    fn default() -> Self {
        LeadStatus::Nuevo
    }
}
