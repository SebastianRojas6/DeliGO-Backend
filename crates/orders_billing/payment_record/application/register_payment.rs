use crate::payment_record::domain::model::{Payment, PaymentStatus, PaymentMethod};
use crate::payment_record::domain::repository::OrdersBillingRepository;

use chrono::Utc;
use std::sync::Arc;

pub struct RegisterPaymentUseCase {
    pub repo: Arc<dyn OrdersBillingRepository>,
}

impl RegisterPaymentUseCase {
    pub fn new(repo: Arc<dyn OrdersBillingRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        order_id: i32,
        total_amount: f64,
        method: PaymentMethod,
    ) -> Result<(), String> {
        if self.repo.get_by_order_id(order_id).await?.is_some() {
            return Err("Ya existe un pago para este pedido.".to_string());
        }

        let payment = Payment {
            id: None,
            order_id,
            total_amount,
            payment_date: Utc::now().naive_utc(),
            payment_status: PaymentStatus::Paid,
            payment_method: method,
        };

        self.repo.register_payment(payment).await
    }
}
