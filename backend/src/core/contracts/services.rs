use crate::api::contracts::dto::CreateContractDtoV2;
use crate::api::contracts::models::{Contract, ContractStatus, ParticipantRole};
use crate::core::domain::audit::{AuditLogEntry, AuditService};
use crate::core::domain::events::{DomainEvent, EventBus};
use crate::core::domain::state_machine::ContractStateMachine;
use crate::infrastructure::database::contracts::ContractRepository;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct ContractService {
    pool: Arc<PgPool>,
    repo: ContractRepository,
}

impl ContractService {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self {
            pool: pool.clone(),
            repo: ContractRepository::new(pool),
        }
    }

    #[tracing::instrument(skip(self, payload), fields(tenant_id = %tenant_id, user_id = %user_id))]
    pub async fn create_contract(
        &self,
        tenant_id: Uuid,
        user_id: Uuid,
        payload: CreateContractDtoV2,
    ) -> Result<Contract, String> {
        // Business Rule: At least 1 main landlord and 1 main tenant
        let has_main_landlord = payload
            .participants
            .iter()
            .any(|p| p.p_role == ParticipantRole::Landlord && p.is_main.unwrap_or(false));
        let has_main_tenant = payload
            .participants
            .iter()
            .any(|p| p.p_role == ParticipantRole::Tenant && p.is_main.unwrap_or(false));

        if !has_main_landlord || !has_main_tenant {
            return Err("Se requiere al menos un Locador principal y un Locatario principal".to_string());
        }

        // Use the repository which handles the transaction, participants, etc.
        // It returns a String error, specifically detecting HTTP 409 for GiST constraint.
        let contract = self
            .repo
            .create_contract_v2(tenant_id, user_id, payload)
            .await?;

        // Inicializar AuditService (cheap to create)
        let audit_service = AuditService::new((*self.pool).clone());
        
        let _ = audit_service.log_event(
            &*self.pool,
            AuditLogEntry {
                tenant_id,
                entity_type: "Contract".to_string(),
                entity_id: contract.id,
                action: "CREATE".to_string(),
                old_payload: None,
                new_payload: serde_json::to_value(&contract).ok(),
                actor_id: user_id,
                ip_address: None,
            }
        ).await;

        // Emit domain event
        crate::core::domain::events::GLOBAL_EVENT_BUS.publish(DomainEvent::ContractDraftCreated {
            contract_id: contract.id,
        });

        Ok(contract)
    }
}
