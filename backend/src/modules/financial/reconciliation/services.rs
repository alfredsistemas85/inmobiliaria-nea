use chrono::Utc;
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::modules::financial::payments::models::Payment;
// We'd import PaymentRepository here if we were querying payments, 
// but for simplicity we will just do direct queries or depend on PaymentService.

use super::{
    models::{BankTransaction, Reconciliation},
    repositories::ReconciliationRepository,
};

pub struct ReconciliationService {
    pool: PgPool,
}

impl ReconciliationService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Auto-reconcile a single bank transaction against existing payments
    /// In a real system, this involves fuzzy matching by amount, date, reference, etc.
    pub async fn auto_reconcile(&self, transaction: BankTransaction) -> Result<Option<Reconciliation>, String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        // Basic matching logic: exact amount and status = COMPLETED or PENDING?
        // Let's assume we find a payment with exact amount that isn't already reconciled
        let matching_payment = sqlx::query_as::<_, Payment>(
            r#"
            SELECT 
                id, tenant_id, account_id, receipt_id, payment_method, 
                payment_reference, amount, currency, payment_date, status, 
                external_provider, external_reference, idempotency_key, 
                created_by, created_at
            FROM payments
            WHERE tenant_id = $1 AND amount = $2
            -- AND we'd ensure it's not already in `reconciliations`
            LIMIT 1
            "#
        )
        .bind(transaction.tenant_id)
        .bind(transaction.amount)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        if let Some(payment) = matching_payment {
            // Create reconciliation
            let rec = Reconciliation {
                id: Uuid::new_v4(),
                payment_id: Some(payment.id),
                bank_transaction_id: Some(transaction.id),
                confidence: rust_decimal_macros::dec!(100.0), // Exact amount match
                matched_by: None, // System
                matched_at: Utc::now(),
            };

            ReconciliationRepository::insert_reconciliation(&mut tx, &rec)
                .await
                .map_err(|e| e.to_string())?;

            ReconciliationRepository::update_transaction_status(&mut tx, transaction.id, "MATCHED")
                .await
                .map_err(|e| e.to_string())?;

            tx.commit().await.map_err(|e| e.to_string())?;
            return Ok(Some(rec));
        }

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(None)
    }
}
