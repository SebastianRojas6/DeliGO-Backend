use async_trait::async_trait;
use super::model::{Payment, Invoice};

#[async_trait]
pub trait PaymentRepository: Send + Sync {
    async fn register_payment(&self, payment: Payment) -> Result<(), String>;

    async fn get_by_order_id(&self, order_id: i32) -> Result<Option<Payment>, String>;
}

#[async_trait]
pub trait InvoiceGenerator: Send + Sync {
    async fn generate_invoice(&self, order_id: i32) -> Result<Invoice, String>;
}