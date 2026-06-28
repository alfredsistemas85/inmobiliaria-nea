pub mod interest_calculator;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum InstallmentStatus {
    PENDING,
    PARTIAL,
    PAID,
    OVERDUE,
    CANCELLED,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PaymentStatus {
    PENDING,
    COMPLETED,
    FAILED,
    REFUNDED,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReceiptStatus {
    DRAFT,
    ISSUED,
    CANCELLED,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CashBoxStatus {
    OPEN,
    CLOSED,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AccountingPeriodStatus {
    OPEN,
    CLOSED,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum InterestCalculationType {
    SIMPLE,
    COMPOUND,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PaymentMethod {
    CASH,
    TRANSFER,
    CREDIT_CARD,
    DEBIT_CARD,
    MERCADO_PAGO,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProviderType {
    LOCAL,
    MERCADO_PAGO,
    AFIP,
    STRIPE,
    BANK_TRANSFER,
}
