use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;
use super::models::Installment;

pub struct InstallmentRepository;

impl InstallmentRepository {
    pub async fn create_installments(
        tx: &mut Transaction<'_, Postgres>,
        installments: &[Installment],
    ) -> Result<(), sqlx::Error> {
        for inst in installments {
            sqlx::query!(
                r#"
                INSERT INTO installments (
                    id, tenant_id, contract_account_id, number, due_date,
                    original_amount, current_amount, interest_amount, paid_amount,
                    remaining_balance, currency, index_value, status, created_at, updated_at
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
                "#,
                inst.id,
                inst.tenant_id,
                inst.contract_account_id,
                inst.number,
                inst.due_date,
                inst.original_amount,
                inst.current_amount,
                inst.interest_amount,
                inst.paid_amount,
                inst.remaining_balance,
                inst.currency,
                inst.index_value,
                inst.status,
                inst.created_at,
                inst.updated_at
            )
            .execute(&mut **tx)
            .await?;
        }
        Ok(())
    }

    pub async fn get_pending_installments_for_update(
        tx: &mut Transaction<'_, Postgres>,
        contract_account_id: Uuid,
    ) -> Result<Vec<Installment>, sqlx::Error> {
        let installments = sqlx::query_as!(
            Installment,
            r#"
            SELECT 
                id, tenant_id, contract_account_id, number, due_date,
                original_amount, current_amount, interest_amount, paid_amount,
                remaining_balance, currency, index_value, status, created_at, updated_at
            FROM installments
            WHERE contract_account_id = $1 AND status IN ('PENDING', 'PARTIAL')
            ORDER BY due_date ASC, number ASC
            FOR UPDATE
            "#,
            contract_account_id
        )
        .fetch_all(&mut **tx)
        .await?;

        Ok(installments)
    }

    pub async fn update_installment(
        tx: &mut Transaction<'_, Postgres>,
        inst: &Installment,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE installments
            SET paid_amount = $1,
                remaining_balance = $2,
                status = $3,
                updated_at = NOW()
            WHERE id = $4
            "#,
            inst.paid_amount,
            inst.remaining_balance,
            inst.status,
            inst.id
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}
