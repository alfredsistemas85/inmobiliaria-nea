use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use super::models::{BankTransaction, Reconciliation};

pub struct ReconciliationRepository;

impl ReconciliationRepository {
    pub async fn insert_bank_transaction(
        pool: &PgPool,
        transaction: &BankTransaction,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO bank_transactions (
                id, tenant_id, account_number, transaction_date, description, 
                amount, reference, reconciliation_status, created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#
        )
        .bind(transaction.id)
        .bind(transaction.tenant_id)
        .bind(&transaction.account_number)
        .bind(transaction.transaction_date)
        .bind(&transaction.description)
        .bind(transaction.amount)
        .bind(&transaction.reference)
        .bind(&transaction.reconciliation_status)
        .bind(transaction.created_at)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn insert_reconciliation(
        tx: &mut Transaction<'_, Postgres>,
        reconciliation: &Reconciliation,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO reconciliations (
                id, payment_id, bank_transaction_id, confidence, matched_by, matched_at
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            "#
        )
        .bind(reconciliation.id)
        .bind(reconciliation.payment_id)
        .bind(reconciliation.bank_transaction_id)
        .bind(reconciliation.confidence)
        .bind(reconciliation.matched_by)
        .bind(reconciliation.matched_at)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn get_unreconciled_transactions(
        pool: &PgPool,
        tenant_id: Uuid,
    ) -> Result<Vec<BankTransaction>, sqlx::Error> {
        let transactions = sqlx::query_as::<_, BankTransaction>(
            r#"
            SELECT 
                id, tenant_id, account_number, transaction_date, description, 
                amount, reference, reconciliation_status, created_at
            FROM bank_transactions
            WHERE tenant_id = $1 AND reconciliation_status = 'PENDING'
            "#
        )
        .bind(tenant_id)
        .fetch_all(pool)
        .await?;

        Ok(transactions)
    }

    pub async fn update_transaction_status(
        tx: &mut Transaction<'_, Postgres>,
        transaction_id: Uuid,
        status: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE bank_transactions
            SET reconciliation_status = $1
            WHERE id = $2
            "#
        )
        .bind(status)
        .bind(transaction_id)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}
