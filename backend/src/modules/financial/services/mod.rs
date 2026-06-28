use sqlx::{PgPool, Transaction, Postgres};
use tracing::instrument;
use uuid::Uuid;

use crate::modules::financial::account::services::AccountService;
use crate::modules::financial::billing::services::BillingService;
use crate::core::domain::events::ContractFinancialTerms;

pub struct FinancialService {
    pool: PgPool,
}

impl FinancialService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    #[instrument(skip(self), fields(tenant_id = %tenant_id, contract_id = %contract_id))]
    pub async fn handle_contract_activated(
        &self,
        tenant_id: Uuid,
        contract_id: Uuid,
        terms: &ContractFinancialTerms,
    ) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        // 1. Open Account
        let account = AccountService::open_contract_account(&mut tx, tenant_id, contract_id).await?;

        // 2. Generate Installments
        BillingService::generate_installments(&mut tx, tenant_id, account.id, terms).await?;

        tx.commit().await?;

        Ok(())
    }
}
