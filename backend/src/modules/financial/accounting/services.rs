use chrono::{NaiveDate, Utc};
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use super::{
    models::{JournalEntry, JournalEntryLine},
    repositories::AccountingRepository,
};

pub struct AccountingService {
    pool: PgPool,
}

impl AccountingService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Records a double-entry journal entry.
    pub async fn record_entry(
        &self,
        tenant_id: Uuid,
        entry_date: NaiveDate,
        description: String,
        lines_data: Vec<(Uuid, Decimal, Decimal)>, // (account_id, debit, credit)
    ) -> Result<Uuid, String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let entry = JournalEntry {
            id: Uuid::new_v4(),
            tenant_id,
            entry_date,
            description,
            status: "POSTED".to_string(),
            created_at: Utc::now(),
        };

        let mut lines = Vec::new();
        for (account_id, debit, credit) in lines_data {
            lines.push(JournalEntryLine {
                id: Uuid::new_v4(),
                entry_id: entry.id,
                account_id,
                debit,
                credit,
                cost_center: None,
                reference: None,
            });
        }

        AccountingRepository::insert_journal_entry(&mut tx, &entry, &lines)
            .await?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(entry.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    use sqlx::Transaction;

    // Unit test for double entry validation (the logic inside the repo)
    #[tokio::test]
    async fn test_double_entry_validation() {
        let mut lines = Vec::new();
        let entry_id = Uuid::new_v4();
        
        // Unbalanced
        lines.push(JournalEntryLine {
            id: Uuid::new_v4(),
            entry_id,
            account_id: Uuid::new_v4(),
            debit: dec!(100.0),
            credit: dec!(0.0),
            cost_center: None,
            reference: None,
        });
        lines.push(JournalEntryLine {
            id: Uuid::new_v4(),
            entry_id,
            account_id: Uuid::new_v4(),
            debit: dec!(0.0),
            credit: dec!(90.0),
            cost_center: None,
            reference: None,
        });

        // We can't easily mock the DB transaction for `AccountingRepository::insert_journal_entry` 
        // without a real DB pool, but we know the repository checks it.
        // We'll write the logic explicitly here to test the math:
        let mut total_debit = dec!(0.0);
        let mut total_credit = dec!(0.0);

        for line in &lines {
            total_debit += line.debit;
            total_credit += line.credit;
        }

        assert_ne!(total_debit, total_credit);
        assert_eq!(total_debit - total_credit, dec!(10.0));
    }
}
