use async_trait::async_trait;
use super::model::{Payment, Invoice};
#[async_trait]
pub trait OrdersBillingRepository: Send + Sync + 'static {
    async fn register_payment(&self, payment: Payment) -> Result<(), String>;
    async fn get_by_order_id(&self, order_id: i32) -> Result<Option<Payment>, String>;
    async fn generate_invoice(&self, order_id: i32) -> Result<Invoice, String>;
}
