use chrono::{Datelike, NaiveDate};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

pub trait InterestCalculator: Send + Sync {
    fn calculate_interest(
        &self, 
        amount: Decimal, 
        due_date: NaiveDate, 
        current_date: NaiveDate
    ) -> Decimal;
}

pub struct SimpleInterestCalculator {
    pub daily_rate: Decimal,
}

impl InterestCalculator for SimpleInterestCalculator {
    fn calculate_interest(
        &self,
        amount: Decimal,
        due_date: NaiveDate,
        current_date: NaiveDate,
    ) -> Decimal {
        if current_date <= due_date {
            return dec!(0.0);
        }
        let days_late = Decimal::from((current_date - due_date).num_days());
        amount * self.daily_rate * days_late
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_simple_interest() {
        let calc = SimpleInterestCalculator { daily_rate: dec!(0.001) }; // 0.1% per day
        let amount = dec!(1000.0);
        let due_date = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        
        let on_time = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        assert_eq!(calc.calculate_interest(amount, due_date, on_time), dec!(0.0));

        let late_10_days = NaiveDate::from_ymd_opt(2026, 1, 11).unwrap();
        // 1000 * 0.001 * 10 = 10.0
        assert_eq!(calc.calculate_interest(amount, due_date, late_10_days), dec!(10.0));
    }
}
