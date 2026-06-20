use sqlx::PgPool;
use std::env;
use rust_decimal::Decimal;
use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "adjustment_method", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AdjustmentMethod {
    Manual,
    FixedPercentage,
    Ipc,
    Icl,
    CasaPropia,
    Custom,
}

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "adjustment_frequency", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AdjustmentFrequency {
    Monthly,
    Bimonthly,
    Quarterly,
    Semester,
    Annual,
}

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "automation_mode", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AutomationMode {
    Manual,
    Semiautomatic,
    Automatic,
}

#[derive(sqlx::FromRow, Debug)]
pub struct Contract {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub property_id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub original_rent_amount: Decimal,
    pub current_rent_amount: Option<Decimal>,
    pub adjustment_method: Option<AdjustmentMethod>,
    pub adjustment_frequency: Option<AdjustmentFrequency>,
    pub automation_mode: Option<AutomationMode>,
    pub fixed_percentage: Option<Decimal>,
    pub first_notification_days: Option<i32>,
    pub second_notification_days: Option<i32>,
    pub third_notification_days: Option<i32>,
    pub requires_manual_approval: Option<bool>,
    pub next_adjustment_date: Option<NaiveDate>,
    pub last_adjustment_date: Option<NaiveDate>,
    pub status: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let db_url = "postgresql://postgres.nwidmlslkkjyvpldzdry:xEnEizE41_2498@aws-1-us-west-2.pooler.supabase.com:5432/postgres";
    let pool = PgPool::connect(db_url).await?;

    let res = sqlx::query_as::<_, Contract>(
        r#"SELECT id, tenant_id, property_id, start_date, end_date, 
           original_rent_amount, current_rent_amount, adjustment_method, adjustment_frequency,
           automation_mode, fixed_percentage, first_notification_days, second_notification_days, third_notification_days, requires_manual_approval, next_adjustment_date, last_adjustment_date, status
           FROM contracts WHERE tenant_id = $1"#
    )
    .bind(Uuid::new_v4())
    .fetch_all(&pool)
    .await;

    println!("{:?}", res);
    Ok(())
}
