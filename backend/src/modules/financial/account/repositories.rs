use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;
use super::models::ContractAccount;

pub struct AccountRepository;

impl AccountRepository {
    pub async fn create_contract_account(
        tx: &mut Transaction<'_, Postgres>,
        account: &ContractAccount,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO contract_accounts (id, tenant_id, contract_id, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            account.id,
            account.tenant_id,
            account.contract_id,
            account.status,
            account.created_at,
            account.updated_at
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}
