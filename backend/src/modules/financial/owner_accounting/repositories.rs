use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use super::models::{OwnerPayment, OwnerStatement, OwnerStatementItem};

pub struct OwnerAccountingRepository;

impl OwnerAccountingRepository {
    pub async fn insert_statement(
        tx: &mut Transaction<'_, Postgres>,
        statement: &OwnerStatement,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO owner_statements (
                id, tenant_id, owner_id, contract_id, period, gross_income, 
                commission_amount, expenses_amount, taxes_amount, net_amount, 
                status, created_at, approved_at, paid_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            "#
        )
        .bind(statement.id)
        .bind(statement.tenant_id)
        .bind(statement.owner_id)
        .bind(statement.contract_id)
        .bind(&statement.period)
        .bind(statement.gross_income)
        .bind(statement.commission_amount)
        .bind(statement.expenses_amount)
        .bind(statement.taxes_amount)
        .bind(statement.net_amount)
        .bind(&statement.status)
        .bind(statement.created_at)
        .bind(statement.approved_at)
        .bind(statement.paid_at)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn insert_statement_items(
        tx: &mut Transaction<'_, Postgres>,
        items: &[OwnerStatementItem],
    ) -> Result<(), sqlx::Error> {
        if items.is_empty() {
            return Ok(());
        }

        let mut qb = sqlx::QueryBuilder::new(
            "INSERT INTO owner_statement_items (id, statement_id, item_type, amount, description) "
        );

        qb.push_values(items, |mut b, item| {
            b.push_bind(item.id)
             .push_bind(item.statement_id)
             .push_bind(&item.item_type)
             .push_bind(item.amount)
             .push_bind(&item.description);
        });

        qb.build().execute(&mut **tx).await?;

        Ok(())
    }

    pub async fn get_pending_statements(
        pool: &PgPool,
        tenant_id: Uuid,
    ) -> Result<Vec<OwnerStatement>, sqlx::Error> {
        sqlx::query_as::<_, OwnerStatement>(
            r#"
            SELECT * FROM owner_statements
            WHERE tenant_id = $1 AND status = 'PENDING'
            "#
        )
        .bind(tenant_id)
        .fetch_all(pool)
        .await
    }

    pub async fn update_statement_status(
        tx: &mut Transaction<'_, Postgres>,
        statement_id: Uuid,
        status: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE owner_statements SET status = $1 WHERE id = $2"
        )
        .bind(status)
        .bind(statement_id)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn insert_owner_payment(
        tx: &mut Transaction<'_, Postgres>,
        payment: &OwnerPayment,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO owner_payments (
                id, statement_id, payment_method, amount, payment_date, reference, status
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#
        )
        .bind(payment.id)
        .bind(payment.statement_id)
        .bind(&payment.payment_method)
        .bind(payment.amount)
        .bind(payment.payment_date)
        .bind(&payment.reference)
        .bind(&payment.status)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}
