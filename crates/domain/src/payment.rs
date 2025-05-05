use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PaymentMethod {
    Visa,
    MasterCard,
    Yape,
    Plin,
    Paypal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PaymentStatus {
    Pending,
    Paid,
    Cancelled,
}

#[derive(Debug, Clone)]
pub struct PaymentEntity {
    pub id_payment: i32,
    pub id_order: i32,
    pub total_amount: f64,
    pub payment_method: PaymentMethod,
    pub payment_status: PaymentStatus,
    pub payment_date: chrono::DateTime<chrono::FixedOffset>,
}

impl PaymentEntity {
    pub fn new(
        id_payment: i32,
        id_order: i32,
        total_amount: f64,
        payment_method: PaymentMethod,
        payment_status: PaymentStatus,
        payment_date: chrono::DateTime<chrono::FixedOffset>,
    ) -> Self {
        Self {
            id_payment,
            id_order,
            total_amount,
            payment_method,
            payment_status,
            payment_date,
        }
    }

    pub fn mark_as_paid(&mut self) {
        self.payment_status = PaymentStatus::Paid;
    }

    pub fn cancel_payment(&mut self) {
        self.payment_status = PaymentStatus::Cancelled;
    }

    pub fn is_paid(&self) -> bool {
        self.payment_status == PaymentStatus::Paid
    }
}