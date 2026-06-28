use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use super::models::{ChartOfAccount, JournalEntry, JournalEntryLine};

pub struct AccountingRepository;

impl AccountingRepository {
    pub async fn insert_journal_entry(
        tx: &mut Transaction<'_, Postgres>,
        entry: &JournalEntry,
        lines: &[JournalEntryLine],
    ) -> Result<(), String> {
        // Validate double-entry accounting rule (Balance = 0)
        let mut total_debit = dec!(0.0);
        let mut total_credit = dec!(0.0);

        for line in lines {
            total_debit += line.debit;
            total_credit += line.credit;
        }

        if total_debit != total_credit {
            return Err(format!("Journal entry is not balanced. Debits: {}, Credits: {}", total_debit, total_credit));
        }

        // Insert entry
        sqlx::query(
            r#"
            INSERT INTO journal_entries (id, tenant_id, entry_date, description, status, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#
        )
        .bind(entry.id)
        .bind(entry.tenant_id)
        .bind(entry.entry_date)
        .bind(&entry.description)
        .bind(&entry.status)
        .bind(entry.created_at)
        .execute(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

        // Insert lines
        let mut qb = sqlx::QueryBuilder::new(
            "INSERT INTO journal_entry_lines (id, entry_id, account_id, debit, credit, cost_center, reference) "
        );

        qb.push_values(lines, |mut b, line| {
            b.push_bind(line.id)
             .push_bind(line.entry_id)
             .push_bind(line.account_id)
             .push_bind(line.debit)
             .push_bind(line.credit)
             .push_bind(&line.cost_center)
             .push_bind(&line.reference);
        });

        qb.build().execute(&mut **tx).await.map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn seed_base_chart_of_accounts(
        pool: &PgPool,
        tenant_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        // Simple seed for initial setup
        let accounts = vec![
            (Uuid::new_v4(), "1.1.01", "Caja", "ASSET"),
            (Uuid::new_v4(), "1.1.02", "Banco", "ASSET"),
            (Uuid::new_v4(), "1.2.01", "Cuentas por Cobrar", "ASSET"),
            (Uuid::new_v4(), "2.1.01", "Cuentas por Pagar Propietarios", "LIABILITY"),
            (Uuid::new_v4(), "4.1.01", "Ingresos por Alquileres", "REVENUE"),
            (Uuid::new_v4(), "4.1.02", "Ingresos por Comisiones", "REVENUE"),
            (Uuid::new_v4(), "5.1.01", "Gastos Bancarios", "EXPENSE"),
        ];

        let mut tx = pool.begin().await?;

        for (id, code, name, type_) in accounts {
            sqlx::query(
                "INSERT INTO chart_of_accounts (id, tenant_id, code, name, account_type) VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING"
            )
            .bind(id)
            .bind(tenant_id)
            .bind(code)
            .bind(name)
            .bind(type_)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }
}
