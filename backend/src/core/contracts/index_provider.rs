use crate::api::contracts::models::AdjustmentMethod;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use async_trait::async_trait;

pub struct IndexCalculation {
    pub source: String,
    pub base_value: Decimal,
    pub current_value: Decimal,
    pub variation_percent: Decimal,
}

#[async_trait]
pub trait IndexProvider: Send + Sync {
    async fn get_index(
        &self,
        index_type: AdjustmentMethod,
        from_date: NaiveDate,
        to_date: NaiveDate,
    ) -> Result<IndexCalculation, String>;
}

pub struct MockIndexProvider;

#[async_trait]
impl IndexProvider for MockIndexProvider {
    async fn get_index(
        &self,
        index_type: AdjustmentMethod,
        _from_date: NaiveDate,
        _to_date: NaiveDate,
    ) -> Result<IndexCalculation, String> {
        let variation = match index_type {
            AdjustmentMethod::Ipc => Decimal::new(2500, 2), // 25.00%
            AdjustmentMethod::Icl => Decimal::new(3000, 2), // 30.00%
            _ => Decimal::new(0, 0),
        };

        Ok(IndexCalculation {
            source: "MOCK_DATA".to_string(),
            base_value: Decimal::new(100, 0),
            current_value: Decimal::new(100, 0) + variation, // just mock
            variation_percent: variation,
        })
    }
}

pub struct ManualIndexProvider;

#[async_trait]
impl IndexProvider for ManualIndexProvider {
    async fn get_index(
        &self,
        _index_type: AdjustmentMethod,
        _from_date: NaiveDate,
        _to_date: NaiveDate,
    ) -> Result<IndexCalculation, String> {
        // Manual index will likely be provided by user input during the "propose" phase,
        // so this provider could return an error saying it requires manual input, or it could
        // fetch it from a "manual_indices" table if they were uploaded.
        Err("Manual index requires operator input".to_string())
    }
}

pub struct FutureApiIndexProvider;

#[async_trait]
impl IndexProvider for FutureApiIndexProvider {
    async fn get_index(
        &self,
        _index_type: AdjustmentMethod,
        _from_date: NaiveDate,
        _to_date: NaiveDate,
    ) -> Result<IndexCalculation, String> {
        Err("Not implemented yet".to_string())
    }
}

pub fn get_provider() -> Box<dyn IndexProvider> {
    let provider_type = std::env::var("INDEX_PROVIDER").unwrap_or_else(|_| "manual".to_string());
    match provider_type.as_str() {
        "mock" => Box::new(MockIndexProvider),
        "api" => Box::new(FutureApiIndexProvider),
        _ => Box::new(ManualIndexProvider),
    }
}
