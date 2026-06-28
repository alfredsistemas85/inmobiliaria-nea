use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use super::models::{Payment, PaymentAllocation, Receipt, ReceiptItem};

pub struct PaymentRepository;

impl PaymentRepository {
    pub async fn check_idempotency(
        pool: &PgPool,
        tenant_id: Uuid,
        idempotency_key: Uuid,
    ) -> Result<Option<Payment>, sqlx::Error> {
        let payment = sqlx::query_as::<_, Payment>(
            r#"
            SELECT 
                id, tenant_id, account_id, receipt_id, payment_method, 
                payment_reference, amount, currency, payment_date, status, 
                external_provider, external_reference, idempotency_key, 
                created_by, created_at
            FROM payments
            WHERE tenant_id = $1 AND idempotency_key = $2
            "#
        )
        .bind(tenant_id)
        .bind(idempotency_key)
        .fetch_optional(pool)
        .await?;

        Ok(payment)
    }

    pub async fn create_payment(
        tx: &mut Transaction<'_, Postgres>,
        payment: &Payment,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO payments (
                id, tenant_id, account_id, receipt_id, payment_method, 
                payment_reference, amount, currency, payment_date, status, 
                external_provider, external_reference, idempotency_key, 
                created_by, created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
            "#
        )
        .bind(payment.id)
        .bind(payment.tenant_id)
        .bind(payment.account_id)
        .bind(payment.receipt_id)
        .bind(&payment.payment_method)
        .bind(&payment.payment_reference)
        .bind(payment.amount)
        .bind(&payment.currency)
        .bind(payment.payment_date)
        .bind(&payment.status)
        .bind(&payment.external_provider)
        .bind(&payment.external_reference)
        .bind(payment.idempotency_key)
        .bind(payment.created_by)
        .bind(payment.created_at)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn create_receipt(
        tx: &mut Transaction<'_, Postgres>,
        receipt: &Receipt,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO receipts (
                id, tenant_id, receipt_number, status, created_at
            )
            VALUES ($1, $2, $3, $4, $5)
            "#,
            receipt.id,
            receipt.tenant_id,
            receipt.receipt_number,
            receipt.status,
            receipt.created_at
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn create_receipt_item(
        tx: &mut Transaction<'_, Postgres>,
        item: &ReceiptItem,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO receipt_items (
                id, receipt_id, description, amount
            )
            VALUES ($1, $2, $3, $4)
            "#,
            item.id,
            item.receipt_id,
            item.description,
            item.amount
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn create_allocations(
        tx: &mut Transaction<'_, Postgres>,
        allocations: &[PaymentAllocation],
    ) -> Result<(), sqlx::Error> {
        if allocations.is_empty() {
            return Ok(());
        }

        let mut qb = sqlx::QueryBuilder::new(
            "INSERT INTO payment_allocations (id, payment_id, installment_id, principal_amount, interest_amount, expense_amount, total_allocated, created_at) "
        );

        qb.push_values(allocations, |mut b, alloc| {
            b.push_bind(alloc.id)
             .push_bind(alloc.payment_id)
             .push_bind(alloc.installment_id)
             .push_bind(alloc.principal_amount)
             .push_bind(alloc.interest_amount)
             .push_bind(alloc.expense_amount)
             .push_bind(alloc.total_allocated)
             .push_bind(alloc.created_at);
        });

        qb.build().execute(&mut **tx).await?;

        Ok(())
    }
}
