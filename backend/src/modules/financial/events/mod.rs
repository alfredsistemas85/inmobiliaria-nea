use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FinancialEvent {
    AccountOpened,
    InstallmentsGenerated,
    InstallmentPaid,
    InstallmentOverdue,
    InterestApplied,
    PaymentReceived,
    ReceiptGenerated,
    OwnerStatementGenerated,
    AccountingEntryCreated,
    CashBoxOpened,
    CashBoxClosed,
}
