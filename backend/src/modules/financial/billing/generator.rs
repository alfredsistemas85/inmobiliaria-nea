use chrono::{Datelike, NaiveDate, Utc};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use uuid::Uuid;

use super::models::Installment;
use crate::core::domain::events::ContractFinancialTerms;

pub struct InstallmentGenerator;

impl InstallmentGenerator {
    pub fn generate_monthly_installments(
        tenant_id: Uuid,
        contract_account_id: Uuid,
        terms: &ContractFinancialTerms,
    ) -> Vec<Installment> {
        let mut installments = Vec::new();
        
        let mut current_date = terms.start_date;
        let mut number = 1;

        // Iterate month by month until we pass the end_date
        while current_date <= terms.end_date {
            // Determine due date (e.g. 10th of the month)
            // For simplicity in this engine, we use the start_date's day or a fixed day.
            let due_date = current_date; 
            
            installments.push(Installment {
                id: Uuid::new_v4(),
                tenant_id,
                contract_account_id,
                number,
                due_date,
                original_amount: terms.amount,
                current_amount: terms.amount,
                interest_amount: dec!(0.0),
                paid_amount: dec!(0.0),
                remaining_balance: terms.amount,
                currency: terms.currency.clone(),
                index_value: None,
                status: "PENDING".to_string(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            });

            // Advance one month
            let mut y = current_date.year();
            let mut m = current_date.month() + 1;
            if m > 12 {
                m = 1;
                y += 1;
            }
            
            // Handle day overflow (e.g. Jan 31 -> Feb 28)
            let mut d = current_date.day();
            loop {
                if let Some(next_date) = NaiveDate::from_ymd_opt(y, m, d) {
                    current_date = next_date;
                    break;
                }
                d -= 1;
                if d == 0 { break; } // Should never happen unless bad date
            }

            number += 1;
        }

        installments
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_generate_monthly_installments() {
        let terms = ContractFinancialTerms {
            tenant_id: Uuid::new_v4(),
            customer_id: Uuid::new_v4(),
            amount: dec!(1000.0),
            currency: "ARS".to_string(),
            start_date: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2026, 12, 31).unwrap(),
            indexation_type: None,
        };

        let account_id = Uuid::new_v4();
        let installments = InstallmentGenerator::generate_monthly_installments(
            terms.tenant_id,
            account_id,
            &terms
        );

        assert_eq!(installments.len(), 12);
        assert_eq!(installments[0].number, 1);
        assert_eq!(installments[0].due_date, NaiveDate::from_ymd_opt(2026, 1, 1).unwrap());
        assert_eq!(installments[11].number, 12);
        assert_eq!(installments[11].due_date, NaiveDate::from_ymd_opt(2026, 12, 1).unwrap());
    }
}
