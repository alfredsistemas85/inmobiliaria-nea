use chrono::Utc;
use rust_decimal::Decimal;
use sqlx::{PgPool, Postgres, Transaction};
use tracing::instrument;
use uuid::Uuid;

use super::models::{Payment, PaymentAllocation, Receipt, ReceiptItem};
use super::repositories::PaymentRepository;
use crate::modules::financial::billing::models::Installment;
use crate::modules::financial::billing::repositories::InstallmentRepository;
use crate::modules::financial::cash::repositories::CashRepository;

pub struct PaymentService {
    pool: PgPool,
}

impl PaymentService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    #[instrument(skip(self), fields(tenant_id = %tenant_id, account_id = %account_id))]
    pub async fn process_payment(
        &self,
        tenant_id: Uuid,
        account_id: Uuid,
        amount: Decimal,
        currency: String,
        payment_method: String,
        idempotency_key: Uuid,
        cash_box_id: Option<Uuid>,
        created_by: Option<Uuid>,
    ) -> Result<Payment, sqlx::Error> {
        // 1. Start Transaction
        let mut tx = self.pool.begin().await?;

        // 2. Check Idempotency
        if let Some(existing_payment) = PaymentRepository::check_idempotency(&self.pool, tenant_id, idempotency_key).await? {
            return Ok(existing_payment);
        }

        // 3. Fetch pending installments and lock for update
        let mut pending_installments = 
            InstallmentRepository::get_pending_installments_for_update(&mut tx, account_id).await?;

        let mut remaining_payment = amount;
        let mut allocations = Vec::new();
        let mut updated_installments = Vec::new();

        // 4. Smart Allocation Loop
        for inst in &mut pending_installments {
            if remaining_payment <= rust_decimal_macros::dec!(0.0) {
                break;
            }

            let mut alloc_principal = rust_decimal_macros::dec!(0.0);
            let mut alloc_interest = rust_decimal_macros::dec!(0.0);
            let mut alloc_expense = rust_decimal_macros::dec!(0.0); // If expenses exist

            // Allocate to interest first
            let unpaid_interest = inst.interest_amount; // Assuming interest_amount is the total interest, wait, we need to track paid_interest too?
            // Actually, we simplified it. Let's assume paid_amount covers interest first, then principal.
            // Let's calculate how much of the inst is already paid.
            let total_due = inst.current_amount + inst.interest_amount;
            let unpaid_total = total_due - inst.paid_amount;

            if unpaid_total <= rust_decimal_macros::dec!(0.0) {
                continue;
            }

            let amount_to_apply = std::cmp::min(remaining_payment, unpaid_total);
            remaining_payment -= amount_to_apply;

            // Simplified breakdown for allocation record
            // First pay off unpaid interest
            // Then pay off unpaid principal
            // In this version, we will just record the total_allocated and a basic breakdown.
            let previously_paid_interest = std::cmp::min(inst.paid_amount, inst.interest_amount);
            let remaining_interest = inst.interest_amount - previously_paid_interest;

            if amount_to_apply >= remaining_interest {
                alloc_interest = remaining_interest;
                alloc_principal = amount_to_apply - remaining_interest;
            } else {
                alloc_interest = amount_to_apply;
            }

            allocations.push(PaymentAllocation {
                id: Uuid::new_v4(),
                payment_id: Uuid::nil(), // Will set after payment is created
                installment_id: inst.id,
                principal_amount: alloc_principal,
                interest_amount: alloc_interest,
                expense_amount: alloc_expense,
                total_allocated: amount_to_apply,
                created_at: Utc::now(),
            });

            inst.paid_amount += amount_to_apply;
            inst.remaining_balance = total_due - inst.paid_amount;
            
            if inst.remaining_balance <= rust_decimal_macros::dec!(0.0) {
                inst.status = "PAID".to_string();
            } else {
                inst.status = "PARTIAL".to_string();
            }

            updated_installments.push(inst.clone());
        }

        // 5. Create Receipt
        let receipt = Receipt {
            id: Uuid::new_v4(),
            tenant_id,
            receipt_number: Some(format!("REC-{}", Uuid::new_v4().to_string().chars().take(8).collect::<String>())),
            status: "ISSUED".to_string(),
            created_at: Utc::now(),
        };
        PaymentRepository::create_receipt(&mut tx, &receipt).await?;

        // 6. Create Payment
        let mut payment = Payment {
            id: Uuid::new_v4(),
            tenant_id,
            account_id,
            receipt_id: Some(receipt.id),
            payment_method,
            payment_reference: None,
            amount,
            currency,
            payment_date: Utc::now(),
            status: "COMPLETED".to_string(),
            external_provider: None,
            external_reference: None,
            idempotency_key: Some(idempotency_key),
            created_by,
            created_at: Utc::now(),
        };
        PaymentRepository::create_payment(&mut tx, &payment).await?;

        // Update allocations with payment_id
        for alloc in &mut allocations {
            alloc.payment_id = payment.id;
        }

        // 7. Save Allocations
        PaymentRepository::create_allocations(&mut tx, &allocations).await?;

        // 8. Update Installments in DB
        for inst in updated_installments {
            InstallmentRepository::update_installment(&mut tx, &inst).await?;
        }

        // 9. Update CashBox if applicable
        if let Some(cash_id) = cash_box_id {
            CashRepository::increment_balance(&mut tx, cash_id, amount).await?;
        }

        tx.commit().await?;

        Ok(payment)
    }
}
