use rust_decimal::Decimal;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

pub struct CashRepository;

impl CashRepository {
    pub async fn increment_balance(
        tx: &mut Transaction<'_, Postgres>,
        cash_box_id: Uuid,
        amount: Decimal,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE cash_boxes
            SET current_balance = current_balance + $1
            WHERE id = $2
            "#,
            amount,
            cash_box_id
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn decrement_balance(
        tx: &mut Transaction<'_, Postgres>,
        cash_box_id: Uuid,
        amount: Decimal,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE cash_boxes
            SET current_balance = current_balance - $1
            WHERE id = $2
            "#,
            amount,
            cash_box_id
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}
