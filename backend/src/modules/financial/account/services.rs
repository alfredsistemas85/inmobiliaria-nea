use chrono::Utc;
use sqlx::{PgPool, Transaction, Postgres};
use tracing::instrument;
use uuid::Uuid;

use super::models::ContractAccount;
use super::repositories::AccountRepository;

pub struct AccountService {
    pool: PgPool,
}

impl AccountService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    #[instrument(skip(tx), fields(tenant_id = %tenant_id, contract_id = %contract_id))]
    pub async fn open_contract_account(
        tx: &mut Transaction<'_, Postgres>,
        tenant_id: Uuid,
        contract_id: Uuid,
    ) -> Result<ContractAccount, sqlx::Error> {
        let account = ContractAccount {
            id: Uuid::new_v4(),
            tenant_id,
            contract_id,
            status: "OPEN".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        AccountRepository::create_contract_account(tx, &account).await?;

        Ok(account)
    }
}
