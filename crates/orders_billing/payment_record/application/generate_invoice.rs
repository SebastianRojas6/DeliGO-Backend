// orders_billing/application/generate_invoice.rs

use crate::payment_record::domain::repository::InvoiceGenerator;
use crate::payment_record::domain::model::Invoice;

use std::sync::Arc;

pub struct GenerateInvoiceUseCase {
    pub repo: Arc<dyn InvoiceGenerator>,
}

impl GenerateInvoiceUseCase {
    pub fn new(repo: Arc<dyn InvoiceGenerator>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, order_id: i32) -> Result<Invoice, String> {
        self.repo.generate_invoice(order_id).await
    }
}
