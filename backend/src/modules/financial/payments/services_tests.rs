#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;
    use chrono::{NaiveDate, Utc};
    use uuid::Uuid;

    use crate::modules::financial::billing::models::Installment;
    use crate::modules::financial::payments::models::PaymentAllocation;

    // We can extract the smart allocation logic into a pure function for easier testing without DB
    pub fn calculate_allocations(
        mut remaining_payment: rust_decimal::Decimal,
        pending_installments: &mut [Installment],
    ) -> Vec<PaymentAllocation> {
        let mut allocations = Vec::new();

        for inst in pending_installments.iter_mut() {
            if remaining_payment <= dec!(0.0) {
                break;
            }

            let mut alloc_principal = dec!(0.0);
            let mut alloc_interest = dec!(0.0);
            let alloc_expense = dec!(0.0);

            let total_due = inst.current_amount + inst.interest_amount;
            let unpaid_total = total_due - inst.paid_amount;

            if unpaid_total <= dec!(0.0) {
                continue;
            }

            let amount_to_apply = std::cmp::min(remaining_payment, unpaid_total);
            remaining_payment -= amount_to_apply;

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
                payment_id: Uuid::nil(),
                installment_id: inst.id,
                principal_amount: alloc_principal,
                interest_amount: alloc_interest,
                expense_amount: alloc_expense,
                total_allocated: amount_to_apply,
                created_at: Utc::now(),
            });

            inst.paid_amount += amount_to_apply;
            inst.remaining_balance = total_due - inst.paid_amount;
            
            if inst.remaining_balance <= dec!(0.0) {
                inst.status = "PAID".to_string();
            } else {
                inst.status = "PARTIAL".to_string();
            }
        }

        allocations
    }

    fn create_dummy_installment(amount: rust_decimal::Decimal, interest: rust_decimal::Decimal) -> Installment {
        Installment {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            contract_account_id: Uuid::new_v4(),
            number: 1,
            due_date: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
            original_amount: amount,
            current_amount: amount,
            interest_amount: interest,
            paid_amount: dec!(0.0),
            remaining_balance: amount + interest,
            currency: "ARS".to_string(),
            index_value: None,
            status: "PENDING".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn test_exact_payment() {
        let mut inst = create_dummy_installment(dec!(1000.0), dec!(100.0));
        let mut installments = vec![inst.clone()];

        let allocs = calculate_allocations(dec!(1100.0), &mut installments);

        assert_eq!(allocs.len(), 1);
        assert_eq!(allocs[0].interest_amount, dec!(100.0));
        assert_eq!(allocs[0].principal_amount, dec!(1000.0));
        
        assert_eq!(installments[0].status, "PAID");
        assert_eq!(installments[0].remaining_balance, dec!(0.0));
    }

    #[test]
    fn test_partial_payment() {
        let mut inst = create_dummy_installment(dec!(1000.0), dec!(100.0));
        let mut installments = vec![inst.clone()];

        let allocs = calculate_allocations(dec!(600.0), &mut installments);

        assert_eq!(allocs.len(), 1);
        assert_eq!(allocs[0].interest_amount, dec!(100.0));
        assert_eq!(allocs[0].principal_amount, dec!(500.0));
        
        assert_eq!(installments[0].status, "PARTIAL");
        assert_eq!(installments[0].remaining_balance, dec!(500.0));
    }

    #[test]
    fn test_excess_payment_cascade() {
        let inst1 = create_dummy_installment(dec!(1000.0), dec!(100.0));
        let mut inst2 = create_dummy_installment(dec!(1000.0), dec!(50.0));
        inst2.number = 2;
        let mut inst3 = create_dummy_installment(dec!(1000.0), dec!(0.0));
        inst3.number = 3;

        let mut installments = vec![inst1, inst2, inst3];

        let allocs = calculate_allocations(dec!(2650.0), &mut installments);

        assert_eq!(allocs.len(), 3);
        
        assert_eq!(installments[0].status, "PAID");
        assert_eq!(installments[1].status, "PAID");
        assert_eq!(installments[2].status, "PARTIAL");
        
        assert_eq!(allocs[2].principal_amount, dec!(500.0));
        assert_eq!(installments[2].remaining_balance, dec!(500.0));
    }
}
