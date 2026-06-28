use chrono::Utc;
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use super::{
    models::TreasuryMovement,
    repositories::TreasuryRepository,
};

pub struct TreasuryService {
    pool: PgPool,
}

impl TreasuryService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Records an inflow or outflow of money, updating the account balance safely.
    pub async fn register_movement(
        &self,
        account_id: Uuid,
        movement_type: &str, // "IN" or "OUT"
        amount: Decimal,
        reference: Option<String>,
        description: String,
    ) -> Result<(), String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        // 1. Lock the account row
        let account = TreasuryRepository::get_account_for_update(&mut tx, account_id)
            .await
            .map_err(|e| e.to_string())?;

        // 2. Calculate new balance
        let new_balance = if movement_type == "IN" {
            account.current_balance + amount
        } else if movement_type == "OUT" {
            account.current_balance - amount
        } else {
            return Err("Invalid movement type".to_string());
        };

        // 3. Update balance
        TreasuryRepository::update_balance(&mut tx, account.id, new_balance)
            .await
            .map_err(|e| e.to_string())?;

        // 4. Insert movement record
        let movement = TreasuryMovement {
            id: Uuid::new_v4(),
            account_id: account.id,
            movement_type: movement_type.to_string(),
            amount,
            reference,
            description,
            created_at: Utc::now(),
        };

        TreasuryRepository::insert_movement(&mut tx, &movement)
            .await
            .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(())
    }

    /// Transfers money between two treasury accounts
    pub async fn transfer(
        &self,
        from_account_id: Uuid,
        to_account_id: Uuid,
        amount: Decimal,
        description: String,
    ) -> Result<(), String> {
        // Simple implementation: OUT then IN.
        // In a real scenario, this would be a single transaction holding two locks in a deterministic order to avoid deadlocks.
        // For demonstration, we'll keep it simple and register two movements.
        
        self.register_movement(from_account_id, "OUT", amount, None, format!("Transfer out: {}", description)).await?;
        self.register_movement(to_account_id, "IN", amount, None, format!("Transfer in: {}", description)).await?;

        Ok(())
    }
}
