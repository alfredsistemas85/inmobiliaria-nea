use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;
use chrono::{NaiveDate, Utc};
use rust_decimal::Decimal;
use crate::core::system_errors::AppError;

pub struct RentalAdjustmentEngine {
    pool: Arc<PgPool>,
}

#[derive(sqlx::FromRow)]
struct AdjRecord {
    contract_id: Uuid,
    previous_amount: Decimal,
    tenant_id: Uuid,
}

impl RentalAdjustmentEngine {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn calculate_new_amount(&self, _contract_id: Uuid, _proposed_index_value: Option<Decimal>) -> Result<Decimal, AppError> {
        // Here we will calculate the new amount. For now, a placeholder logic:
        // get contract's current_rent_amount, and apply proposed index variation
        Ok(Decimal::new(0, 0))
    }

    pub async fn approve_adjustment(&self, adjustment_id: Uuid, approved_by: Uuid, new_amount: Decimal, notes: Option<String>) -> Result<(), AppError> {
        let mut tx = self.pool.begin().await.map_err(|_: sqlx::Error| AppError::InternalServerError)?;

        // 1. Update rent_adjustment status to APPROVED
        let adj_record = sqlx::query_as::<_, AdjRecord>(
            r#"
            UPDATE rent_adjustments
            SET status = 'APPROVED', new_amount = $1, approved_by = $2, approved_at = $3, notes = $4
            WHERE id = $5 AND status = 'PENDING'
            RETURNING contract_id, previous_amount, tenant_id
            "#
        )
        .bind(new_amount)
        .bind(approved_by)
        .bind(Utc::now())
        .bind(notes)
        .bind(adjustment_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|_: sqlx::Error| AppError::NotFoundError)?;

        // 2. Update contract current_rent_amount
        sqlx::query(
            r#"
            UPDATE contracts
            SET current_rent_amount = $1, last_adjustment_date = CURRENT_DATE
            WHERE id = $2
            "#
        )
        .bind(new_amount)
        .bind(adj_record.contract_id)
        .execute(&mut *tx)
        .await
        .map_err(|_: sqlx::Error| AppError::InternalServerError)?;

        // 3. Recalculate pending installments
        sqlx::query(
            r#"
            UPDATE contract_installments
            SET amount = $1, updated_at = CURRENT_TIMESTAMP
            WHERE contract_id = $2 AND status = 'PENDING' AND due_date >= CURRENT_DATE
            "#
        )
        .bind(new_amount)
        .bind(adj_record.contract_id)
        .execute(&mut *tx)
        .await
        .map_err(|_: sqlx::Error| AppError::InternalServerError)?;

        // 4. Create Audit Log
        let old_data = serde_json::json!({ "amount": adj_record.previous_amount });
        let new_data = serde_json::json!({ "amount": new_amount });

        sqlx::query(
            r#"
            INSERT INTO audit_logs (tenant_id, user_id, contract_id, action, old_data, new_data, method)
            VALUES ($1, $2, $3, 'APPROVE_RENT_ADJUSTMENT', $4, $5, 'POST')
            "#
        )
        .bind(adj_record.tenant_id)
        .bind(approved_by)
        .bind(adj_record.contract_id)
        .bind(old_data)
        .bind(new_data)
        .execute(&mut *tx)
        .await
        .map_err(|_: sqlx::Error| AppError::InternalServerError)?;

        tx.commit().await.map_err(|_: sqlx::Error| AppError::InternalServerError)?;

        // 5. Queue event for PDF generation and WhatsApp notification
        let event = crate::core::contracts::events::RentAdjustmentApproved {
            tenant_id: adj_record.tenant_id,
            contract_id: adj_record.contract_id,
            adjustment_id,
            user_id: approved_by,
        };

        let pool_clone = self.pool.clone();
        tokio::spawn(async move {
            crate::core::contracts::events::handle_rent_adjustment_approved(event, pool_clone).await;
        });

        Ok(())
    }

    pub async fn rollback_adjustment(&self, adjustment_id: Uuid, user_id: Uuid, reason: String) -> Result<(), AppError> {
        let mut tx = self.pool.begin().await.map_err(|_: sqlx::Error| AppError::InternalServerError)?;

        let adj_record = sqlx::query_as::<_, AdjRecord>(
            r#"
            UPDATE rent_adjustments
            SET status = 'ROLLED_BACK', rollback_reason = $1
            WHERE id = $2 AND status = 'APPROVED'
            RETURNING contract_id, previous_amount, tenant_id
            "#
        )
        .bind(&reason)
        .bind(adjustment_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|_: sqlx::Error| AppError::NotFoundError)?;

        sqlx::query(
            r#"
            UPDATE contracts
            SET current_rent_amount = $1
            WHERE id = $2
            "#
        )
        .bind(adj_record.previous_amount)
        .bind(adj_record.contract_id)
        .execute(&mut *tx)
        .await
        .map_err(|_: sqlx::Error| AppError::InternalServerError)?;

        sqlx::query(
            r#"
            UPDATE contract_installments
            SET amount = $1, updated_at = CURRENT_TIMESTAMP
            WHERE contract_id = $2 AND status = 'PENDING' AND due_date >= CURRENT_DATE
            "#
        )
        .bind(adj_record.previous_amount)
        .bind(adj_record.contract_id)
        .execute(&mut *tx)
        .await
        .map_err(|_: sqlx::Error| AppError::InternalServerError)?;

        let old_data = serde_json::json!({ "reason": reason });
        let new_data = serde_json::json!({ "amount": adj_record.previous_amount });

        sqlx::query(
            r#"
            INSERT INTO audit_logs (tenant_id, user_id, contract_id, action, old_data, new_data, method)
            VALUES ($1, $2, $3, 'ROLLBACK_RENT_ADJUSTMENT', $4, $5, 'POST')
            "#
        )
        .bind(adj_record.tenant_id)
        .bind(user_id)
        .bind(adj_record.contract_id)
        .bind(old_data)
        .bind(new_data)
        .execute(&mut *tx)
        .await
        .map_err(|_: sqlx::Error| AppError::InternalServerError)?;

        tx.commit().await.map_err(|_: sqlx::Error| AppError::InternalServerError)?;

        Ok(())
    }
}
