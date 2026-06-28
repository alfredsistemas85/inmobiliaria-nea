use rust_decimal::Decimal;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use super::models::{TreasuryAccount, TreasuryMovement};

pub struct TreasuryRepository;

impl TreasuryRepository {
    pub async fn create_account(
        pool: &PgPool,
        account: &TreasuryAccount,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO treasury_accounts (id, tenant_id, name, type, currency, current_balance, status)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#
        )
        .bind(account.id)
        .bind(account.tenant_id)
        .bind(&account.name)
        .bind(&account.account_type)
        .bind(&account.currency)
        .bind(account.current_balance)
        .bind(&account.status)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get_account_for_update(
        tx: &mut Transaction<'_, Postgres>,
        account_id: Uuid,
    ) -> Result<TreasuryAccount, sqlx::Error> {
        sqlx::query_as::<_, TreasuryAccount>(
            r#"
            SELECT id, tenant_id, name, type, currency, current_balance, status
            FROM treasury_accounts
            WHERE id = $1
            FOR UPDATE
            "#
        )
        .bind(account_id)
        .fetch_one(&mut **tx)
        .await
    }

    pub async fn update_balance(
        tx: &mut Transaction<'_, Postgres>,
        account_id: Uuid,
        new_balance: Decimal,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE treasury_accounts SET current_balance = $1 WHERE id = $2"
        )
        .bind(new_balance)
        .bind(account_id)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn insert_movement(
        tx: &mut Transaction<'_, Postgres>,
        movement: &TreasuryMovement,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO treasury_movements (id, account_id, movement_type, amount, reference, description, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#
        )
        .bind(movement.id)
        .bind(movement.account_id)
        .bind(&movement.movement_type)
        .bind(movement.amount)
        .bind(&movement.reference)
        .bind(&movement.description)
        .bind(movement.created_at)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}
