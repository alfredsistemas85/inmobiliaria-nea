use sqlx::PgPool;
use tracing::instrument;
use std::sync::Arc;
use crate::modules::financial::shared::interest_calculator::{InterestCalculator, SimpleInterestCalculator};

use rust_decimal_macros::dec;

pub struct DailyScheduler {
    pool: PgPool,
    calculator: Arc<dyn InterestCalculator>,
}

impl DailyScheduler {
    pub fn new(pool: PgPool) -> Self {
        Self { 
            pool,
            calculator: Arc::new(SimpleInterestCalculator { daily_rate: dec!(0.001) }) // Default 0.1% for now
        }
    }

    #[instrument(skip(self))]
    pub async fn process_overdue_installments(&self) -> Result<(), sqlx::Error> {
        // Here we would fetch all PENDING or PARTIAL installments where due_date < current_date
        // calculate interest using self.calculator
        // and update their current_amount and status to OVERDUE.
        
        Ok(())
    }
}
