use std::sync::Arc;

pub trait PaymentProvider: Send + Sync {
    // Skeleton trait
}

pub trait InvoiceProvider: Send + Sync {
    // Skeleton trait
}

pub trait NotificationProvider: Send + Sync {
    // Skeleton trait
}

pub trait StorageProvider: Send + Sync {
    // Skeleton trait
}

pub struct MockPaymentProvider;
impl PaymentProvider for MockPaymentProvider {}

pub struct MockInvoiceProvider;
impl InvoiceProvider for MockInvoiceProvider {}

pub struct MockNotificationProvider;
impl NotificationProvider for MockNotificationProvider {}

pub struct LocalStorageProvider;
impl StorageProvider for LocalStorageProvider {}

pub struct ProviderFactory;

impl ProviderFactory {
    pub fn get_payment_provider() -> Arc<dyn PaymentProvider> {
        Arc::new(MockPaymentProvider)
    }

    pub fn get_invoice_provider() -> Arc<dyn InvoiceProvider> {
        Arc::new(MockInvoiceProvider)
    }

    pub fn get_notification_provider() -> Arc<dyn NotificationProvider> {
        Arc::new(MockNotificationProvider)
    }

    pub fn get_storage_provider() -> Arc<dyn StorageProvider> {
        Arc::new(LocalStorageProvider)
    }
}
