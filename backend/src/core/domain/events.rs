use serde::{Deserialize, Serialize};
use uuid::Uuid;

use chrono::NaiveDate;
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractFinancialTerms {
    pub tenant_id: Uuid,
    pub customer_id: Uuid,
    pub amount: Decimal,
    pub currency: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub indexation_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DomainEvent {
    ContractDraftCreated { contract_id: Uuid },
    ContractSnapshotFrozen { contract_id: Uuid },
    SignatureRequested { contract_id: Uuid },
    ContractActivated { contract_id: Uuid, terms: ContractFinancialTerms },
    ContractCancelled { contract_id: Uuid },
    ContractRenewed { old_contract_id: Uuid, new_contract_id: Uuid },
    ContractExpired { contract_id: Uuid },
}

pub struct EventBus {
    sender: tokio::sync::broadcast::Sender<DomainEvent>,
}

impl EventBus {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = tokio::sync::broadcast::channel(capacity);
        Self { sender }
    }

    pub fn publish(&self, event: DomainEvent) {
        // En un entorno de producción, los errores de envío por falta de receptores
        // pueden ser ignorados si no hay módulos escuchando activamente.
        let _ = self.sender.send(event);
    }

    pub fn subscribe(&self) -> tokio::sync::broadcast::Receiver<DomainEvent> {
        self.sender.subscribe()
    }
}

use std::sync::LazyLock;

pub static GLOBAL_EVENT_BUS: LazyLock<EventBus> = LazyLock::new(|| EventBus::new(100));
