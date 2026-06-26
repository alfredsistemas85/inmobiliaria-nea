use chrono::Utc;
use sqlx::PgPool;
use std::sync::Arc;
use tracing::{error, info, warn};

use crate::core::contracts::adjustment_engine::RentalAdjustmentEngine;
use crate::core::system_errors::AppError;

pub struct RentalAdjustmentScheduler {
    pool: Arc<PgPool>,
    engine: Arc<RentalAdjustmentEngine>,
    redis_client: Option<redis::Client>,
}

use serde::Serialize;

#[derive(Serialize)]
pub struct SchedulerMetrics {
    pub contracts_checked: u64,
    pub adjustments_generated: u64,
    pub execution_time_ms: u128,
}

impl RentalAdjustmentScheduler {
    pub fn new(pool: Arc<PgPool>, engine: Arc<RentalAdjustmentEngine>) -> Self {
        let redis_url =
            std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
        let redis_client = redis::Client::open(redis_url).ok();

        Self {
            pool,
            engine,
            redis_client,
        }
    }

    pub async fn process_daily_adjustments(&self) -> Result<SchedulerMetrics, AppError> {
        let start_time = std::time::Instant::now();
        let today = Utc::now().naive_utc().date();
        let lock_key = format!("adjustment_scheduler:{}", today);

        // 1. Redis Lock
        if let Some(ref client) = self.redis_client {
            if let Ok(mut con) = client.get_async_connection().await {
                let is_set: bool = redis::cmd("SET")
                    .arg(&lock_key)
                    .arg("LOCKED")
                    .arg("NX")
                    .arg("EX")
                    .arg(48 * 3600) // 48 hours TTL
                    .query_async(&mut con)
                    .await
                    .unwrap_or(false);

                if !is_set {
                    info!("Scheduler already ran today according to Redis lock. Skipping.");
                    return Ok(SchedulerMetrics {
                        contracts_checked: 0,
                        adjustments_generated: 0,
                        execution_time_ms: 0,
                    });
                }
            }
        }

        // 2. PostgreSQL Lock
        let pg_lock = sqlx::query(
            "INSERT INTO scheduler_runs (process_name, execution_date) VALUES ('DAILY_RENT_ADJUSTMENTS', $1) ON CONFLICT DO NOTHING"
        )
        .bind(today)
        .execute(&*self.pool)
        .await
        .map_err(|_| AppError::InternalServerError)?;

        if pg_lock.rows_affected() == 0 {
            info!("Scheduler already ran today according to PostgreSQL lock. Skipping.");
            return Ok(SchedulerMetrics {
                contracts_checked: 0,
                adjustments_generated: 0,
                execution_time_ms: 0,
            });
        }

        info!("Starting daily rent adjustments processing for {}", today);

        // 3. Find candidate contracts
        #[derive(sqlx::FromRow)]
        struct Candidate {
            id: uuid::Uuid,
            tenant_id: uuid::Uuid,
            next_adjustment_date: Option<chrono::NaiveDate>,
            adjustment_method: String,
            automation_mode: String,
        }

        let candidates = sqlx::query_as::<_, Candidate>(
            r#"
            SELECT c.id, c.tenant_id, c.next_adjustment_date, c.adjustment_method::text, c.automation_mode::text
            FROM contracts c
            WHERE c.next_adjustment_date IS NOT NULL 
              AND CURRENT_DATE >= (c.next_adjustment_date - c.first_notification_days)
              AND c.automation_mode IN ('SEMIAUTOMATIC', 'AUTOMATIC')
            "#
        )
        .fetch_all(&*self.pool)
        .await
        .map_err(|_| AppError::InternalServerError)?;

        info!(
            "Found {} candidate contracts for adjustment",
            candidates.len()
        );

        let automatic_enabled = std::env::var("ENABLE_AUTOMATIC_ADJUSTMENTS")
            .unwrap_or_else(|_| "false".to_string())
            == "true";

        let mut adjustments_generated = 0;

        for contract in &candidates {
            if let Some(date) = contract.next_adjustment_date {
                match self
                    .engine
                    .propose_system_adjustment(contract.id, date, &contract.adjustment_method)
                    .await
                {
                    Ok(adj_id) => {
                        info!("Generated proposal {} for contract {}", adj_id, contract.id);
                        adjustments_generated += 1;

                        // Automatic Approval Logic
                        if contract.automation_mode == "AUTOMATIC" && automatic_enabled {
                            // If index data is missing, we leave it PENDING_INDEX_DATA, else we can approve it.
                            // approve_system_adjustment handles it.
                            if let Err(e) = self.engine.approve_system_adjustment(adj_id).await {
                                warn!("Failed to auto-approve adjustment {}: {:?}", adj_id, e);
                            }
                        }
                    }
                    Err(e) => {
                        warn!(
                            "Failed to propose adjustment for contract {}: {:?}",
                            contract.id, e
                        );
                    }
                }
            }
        }

        // Log completion audit
        let _ = sqlx::query(
            "INSERT INTO audit_logs (tenant_id, user_id, contract_id, action, old_data, new_data, method)
             VALUES ($1, $2, $3, 'ADJUSTMENT_SCHEDULER_EXECUTED', '{}', '{}', 'SYSTEM')"
        )
        .bind(uuid::Uuid::nil()) // System action
        .bind(uuid::Uuid::nil()) // System action
        .bind(uuid::Uuid::nil()) // System action
        .execute(&*self.pool)
        .await;

        info!("Finished daily rent adjustments processing for {}", today);
        Ok(SchedulerMetrics {
            contracts_checked: candidates.len() as u64,
            adjustments_generated,
            execution_time_ms: start_time.elapsed().as_millis(),
        })
    }
}
