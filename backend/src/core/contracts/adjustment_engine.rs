use crate::core::system_errors::AppError;
use chrono::{NaiveDate, Utc};
use rust_decimal::Decimal;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

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

    pub async fn calculate_new_amount(
        &self,
        contract_id: Uuid,
        proposed_index_value: Option<Decimal>,
    ) -> Result<Decimal, AppError> {
        #[derive(sqlx::FromRow)]
        struct ContractData {
            current_rent_amount: Option<Decimal>,
            adjustment_method: Option<crate::api::contracts::models::AdjustmentMethod>,
            fixed_percentage: Option<Decimal>,
        }

        let contract = sqlx::query_as::<_, ContractData>(
            "SELECT current_rent_amount, adjustment_method, fixed_percentage FROM contracts WHERE id = $1"
        )
        .bind(contract_id)
        .fetch_one(&*self.pool)
        .await
        .map_err(|_| AppError::NotFoundError)?;

        let current_amount = contract.current_rent_amount.unwrap_or(Decimal::new(0, 0));
        let method = contract
            .adjustment_method
            .unwrap_or(crate::api::contracts::models::AdjustmentMethod::Manual);

        let new_amount = match method {
            crate::api::contracts::models::AdjustmentMethod::Manual
            | crate::api::contracts::models::AdjustmentMethod::Custom => {
                proposed_index_value.unwrap_or(current_amount)
            }
            crate::api::contracts::models::AdjustmentMethod::FixedPercentage => {
                let percentage = contract.fixed_percentage.unwrap_or(Decimal::new(0, 0));
                current_amount + (current_amount * percentage / Decimal::new(100, 0))
            }
            _ => {
                let provider = crate::core::contracts::index_provider::get_provider();
                let today = chrono::Utc::now().naive_utc().date();
                match provider.get_index(method.clone(), today, today).await {
                    Ok(calc) => {
                        current_amount
                            + (current_amount * calc.variation_percent / Decimal::new(100, 0))
                    }
                    Err(_) => current_amount,
                }
            }
        };

        Ok(new_amount)
    }

    pub async fn approve_adjustment(
        &self,
        adjustment_id: Uuid,
        approved_by: Uuid,
        new_amount: Decimal,
        notes: Option<String>,
    ) -> Result<(), AppError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|_: sqlx::Error| AppError::InternalServerError)?;

        // 1. Update rent_adjustment status to APPROVED
        let adj_record = sqlx::query_as::<_, AdjRecord>(
            r#"
            UPDATE rent_adjustments
            SET status = 'APPROVED', new_amount = $1, approved_by = $2, approved_at = $3, notes = $4
            WHERE id = $5 AND status = 'PENDING'
            RETURNING contract_id, previous_amount, tenant_id
            "#,
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
            "#,
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
            "#,
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

        tx.commit()
            .await
            .map_err(|_: sqlx::Error| AppError::InternalServerError)?;

        // 5. Queue event for PDF generation and WhatsApp notification
        let event = crate::core::contracts::events::RentAdjustmentApproved {
            tenant_id: adj_record.tenant_id,
            contract_id: adj_record.contract_id,
            adjustment_id,
            user_id: approved_by,
        };

        let pool_clone = self.pool.clone();
        tokio::spawn(async move {
            crate::core::contracts::events::handle_rent_adjustment_approved(event, pool_clone)
                .await;
        });

        Ok(())
    }

    pub async fn reject_adjustment(
        &self,
        adjustment_id: Uuid,
        user_id: Uuid,
        reason: String,
    ) -> Result<(), AppError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|_: sqlx::Error| AppError::InternalServerError)?;

        let adj_record = sqlx::query_as::<_, AdjRecord>(
            r#"
            UPDATE rent_adjustments
            SET status = 'REJECTED', rollback_reason = $1
            WHERE id = $2 AND status = 'PENDING'
            RETURNING contract_id, previous_amount, tenant_id
            "#,
        )
        .bind(&reason)
        .bind(adjustment_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|_: sqlx::Error| AppError::NotFoundError)?;

        let old_data = serde_json::json!({ "status": "PENDING" });
        let new_data = serde_json::json!({ "status": "REJECTED", "reason": reason });

        sqlx::query(
            r#"
            INSERT INTO audit_logs (tenant_id, user_id, contract_id, action, old_data, new_data, method)
            VALUES ($1, $2, $3, 'REJECT_RENT_ADJUSTMENT', $4, $5, 'POST')
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

        tx.commit()
            .await
            .map_err(|_: sqlx::Error| AppError::InternalServerError)?;

        Ok(())
    }

    pub async fn propose_system_adjustment(
        &self,
        contract_id: Uuid,
        effective_date: NaiveDate,
        method: &str,
    ) -> Result<Uuid, AppError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|_| AppError::InternalServerError)?;

        #[derive(sqlx::FromRow)]
        struct ContractRecord {
            tenant_id: Uuid,
            current_rent_amount: Option<Decimal>,
        }

        let contract_record = sqlx::query_as::<_, ContractRecord>(
            "SELECT tenant_id, current_rent_amount FROM contracts WHERE id = $1",
        )
        .bind(contract_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|_| AppError::NotFoundError)?;

        let adjustment_method = match method {
            "IPC" => crate::api::contracts::models::AdjustmentMethod::Ipc,
            "ICL" => crate::api::contracts::models::AdjustmentMethod::Icl,
            "CASA_PROPIA" => crate::api::contracts::models::AdjustmentMethod::CasaPropia,
            "FIXED_PERCENTAGE" => crate::api::contracts::models::AdjustmentMethod::FixedPercentage,
            "CUSTOM" => crate::api::contracts::models::AdjustmentMethod::Custom,
            _ => crate::api::contracts::models::AdjustmentMethod::Manual,
        };

        let provider = crate::core::contracts::index_provider::get_provider();
        let index_calc = provider
            .get_index(adjustment_method, effective_date, effective_date)
            .await;

        let (status, new_amount, percentage_applied) = match index_calc {
            Ok(calc) => {
                let new_amt = contract_record
                    .current_rent_amount
                    .unwrap_or(Decimal::new(0, 0))
                    * (Decimal::new(1, 0) + calc.variation_percent);
                ("PENDING", new_amt, Some(calc.variation_percent))
            }
            Err(_) => ("PENDING_INDEX_DATA", Decimal::new(0, 0), None),
        };

        let adjustment_id = Uuid::new_v4();

        sqlx::query(
            r#"
            INSERT INTO rent_adjustments (id, tenant_id, contract_id, adjustment_method, status, previous_amount, new_amount, percentage_applied, effective_date)
            VALUES ($1, $2, $3, $4::adjustment_method, $5::adjustment_status, $6, $7, $8, $9)
            "#
        )
        .bind(adjustment_id)
        .bind(contract_record.tenant_id)
        .bind(contract_id)
        .bind(method)
        .bind(status)
        .bind(contract_record.current_rent_amount)
        .bind(new_amount)
        .bind(percentage_applied)
        .bind(effective_date)
        .execute(&mut *tx)
        .await
        .map_err(|_| AppError::InternalServerError)?;

        let msg = if status == "PENDING_INDEX_DATA" {
            format!(
                "No fue posible obtener el índice para el contrato {}.",
                contract_id
            )
        } else {
            format!(
                "Existe un ajuste de alquiler pendiente de revisión para el contrato {}.",
                contract_id
            )
        };

        let _ = sqlx::query(
            "INSERT INTO notifications (tenant_id, message, type) VALUES ($1, $2, 'SYSTEM_ALERT')",
        )
        .bind(contract_record.tenant_id)
        .bind(msg)
        .execute(&mut *tx)
        .await;

        let _ = sqlx::query(
            "INSERT INTO audit_logs (tenant_id, user_id, contract_id, action, old_data, new_data, method)
             VALUES ($1, $2, $3, 'ADJUSTMENT_PROPOSAL_CREATED', '{}', '{}', 'SYSTEM')"
        ).bind(contract_record.tenant_id).bind(Uuid::nil()).bind(contract_id).execute(&mut *tx).await;

        tx.commit()
            .await
            .map_err(|_| AppError::InternalServerError)?;

        Ok(adjustment_id)
    }

    pub async fn approve_system_adjustment(&self, adjustment_id: Uuid) -> Result<(), AppError> {
        #[derive(sqlx::FromRow)]
        struct StatusRecord {
            new_amount: Decimal,
            status: String,
        }

        let record = sqlx::query_as::<_, StatusRecord>(
            "SELECT new_amount, status::text FROM rent_adjustments WHERE id = $1",
        )
        .bind(adjustment_id)
        .fetch_one(&*self.pool)
        .await
        .map_err(|_| AppError::NotFoundError)?;

        if record.status != "PENDING" {
            return Err(AppError::BadRequest("Not pending".to_string()));
        }

        self.approve_adjustment(
            adjustment_id,
            Uuid::nil(),
            record.new_amount,
            Some("Auto-approved by system".to_string()),
        )
        .await
    }
}
