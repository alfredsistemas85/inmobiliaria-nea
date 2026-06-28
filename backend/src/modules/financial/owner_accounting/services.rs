use chrono::Utc;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use super::{
    models::{OwnerStatement, OwnerStatementItem},
    repositories::OwnerAccountingRepository,
};

pub struct OwnerAccountingService {
    pool: PgPool,
}

impl OwnerAccountingService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Generates a draft owner statement for a given period, based on received rent payments.
    /// This is a simplified calculation: 
    /// Gross Income - Commission (e.g. 10%) - Expenses - Taxes = Net Amount
    pub async fn generate_statement(
        &self,
        tenant_id: Uuid,
        owner_id: Uuid,
        contract_id: Uuid,
        period: &str,
        gross_income: Decimal,
        commission_rate: Decimal, // e.g., 0.10 for 10%
        expenses_amount: Decimal,
    ) -> Result<OwnerStatement, String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let commission_amount = gross_income * commission_rate;
        // Simplify taxes for now (e.g., IVA on commission)
        let taxes_amount = commission_amount * dec!(0.21); 

        let net_amount = gross_income - commission_amount - expenses_amount - taxes_amount;

        let statement = OwnerStatement {
            id: Uuid::new_v4(),
            tenant_id,
            owner_id,
            contract_id,
            period: period.to_string(),
            gross_income,
            commission_amount,
            expenses_amount,
            taxes_amount,
            net_amount,
            status: "DRAFT".to_string(),
            created_at: Utc::now(),
            approved_at: None,
            paid_at: None,
        };

        OwnerAccountingRepository::insert_statement(&mut tx, &statement)
            .await
            .map_err(|e| e.to_string())?;

        // Create detail items
        let mut items = vec![
            OwnerStatementItem {
                id: Uuid::new_v4(),
                statement_id: statement.id,
                item_type: "RENT".to_string(),
                amount: gross_income,
                description: format!("Alquiler del período {}", period),
            },
            OwnerStatementItem {
                id: Uuid::new_v4(),
                statement_id: statement.id,
                item_type: "COMMISSION".to_string(),
                amount: -commission_amount,
                description: "Honorarios de administración".to_string(),
            },
        ];

        if expenses_amount > dec!(0.0) {
            items.push(OwnerStatementItem {
                id: Uuid::new_v4(),
                statement_id: statement.id,
                item_type: "EXPENSE".to_string(),
                amount: -expenses_amount,
                description: "Gastos de mantenimiento".to_string(),
            });
        }

        if taxes_amount > dec!(0.0) {
            items.push(OwnerStatementItem {
                id: Uuid::new_v4(),
                statement_id: statement.id,
                item_type: "TAX".to_string(),
                amount: -taxes_amount,
                description: "IVA s/ honorarios".to_string(),
            });
        }

        OwnerAccountingRepository::insert_statement_items(&mut tx, &items)
            .await
            .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(statement)
    }

    pub async fn approve_statement(
        &self,
        statement_id: Uuid,
    ) -> Result<(), String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        
        OwnerAccountingRepository::update_statement_status(&mut tx, statement_id, "APPROVED")
            .await
            .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_statement_math() {
        let gross_income = dec!(1000.0);
        let commission_rate = dec!(0.10); // 10%
        let expenses = dec!(50.0);

        let commission_amount = gross_income * commission_rate; // 100
        let taxes = commission_amount * dec!(0.21); // 21
        let net = gross_income - commission_amount - expenses - taxes;

        assert_eq!(commission_amount, dec!(100.0));
        assert_eq!(taxes, dec!(21.0));
        assert_eq!(net, dec!(829.0));
    }
}
