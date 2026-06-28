use crate::api::contracts::models::ContractStatus;
use super::errors::DomainError;

pub struct ContractStateMachine;

impl ContractStateMachine {
    /// Valida si la transición de un estado a otro es válida
    #[tracing::instrument(skip_all, fields(from = ?from, to = ?to))]
    pub fn validate_transition(from: &ContractStatus, to: &ContractStatus) -> Result<(), DomainError> {
        let is_valid = match (from, to) {
            // Desde DRAFT
            (ContractStatus::Draft, ContractStatus::PendingSignature) => true,
            (ContractStatus::Draft, ContractStatus::Annulled) => true,
            
            // Desde PENDING_SIGNATURE
            (ContractStatus::PendingSignature, ContractStatus::Active) => true,
            (ContractStatus::PendingSignature, ContractStatus::Signed) => true, // En caso de requerir paso intermedio
            (ContractStatus::PendingSignature, ContractStatus::Annulled) => true,

            // Desde SIGNED
            (ContractStatus::Signed, ContractStatus::Active) => true,
            (ContractStatus::Signed, ContractStatus::Annulled) => true,

            // Desde ACTIVE
            (ContractStatus::Active, ContractStatus::Suspended) => true,
            (ContractStatus::Active, ContractStatus::Terminated) => true,
            (ContractStatus::Active, ContractStatus::Finished) => true, // Equivalent to EXPIRED

            // Desde SUSPENDED
            (ContractStatus::Suspended, ContractStatus::Active) => true,
            (ContractStatus::Suspended, ContractStatus::Terminated) => true,

            // Estados finales (no permiten transiciones salientes)
            (ContractStatus::Annulled, _) => false,
            (ContractStatus::Terminated, _) => false,
            (ContractStatus::Finished, _) => false,

            _ => false,
        };

        if is_valid {
            Ok(())
        } else {
            Err(DomainError::InvalidStateTransition {
                from: format!("{:?}", from),
                to: format!("{:?}", to),
                entity: "Contract".to_string(),
            })
        }
    }
}
