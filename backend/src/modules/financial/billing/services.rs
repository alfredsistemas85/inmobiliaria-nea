use sqlx::{PgPool, Transaction, Postgres};
use tracing::instrument;
use uuid::Uuid;

use super::generator::InstallmentGenerator;
use super::repositories::InstallmentRepository;
use crate::core::domain::events::ContractFinancialTerms;

pub struct BillingService {
    pool: PgPool,
}

impl BillingService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    #[instrument(skip(tx), fields(tenant_id = %tenant_id, account_id = %contract_account_id))]
    pub async fn generate_installments(
        tx: &mut Transaction<'_, Postgres>,
        tenant_id: Uuid,
        contract_account_id: Uuid,
        terms: &ContractFinancialTerms,
    ) -> Result<(), sqlx::Error> {
        let installments = InstallmentGenerator::generate_monthly_installments(
            tenant_id,
            contract_account_id,
            terms,
        );

        InstallmentRepository::create_installments(tx, &installments).await?;

        Ok(())
    }
}
