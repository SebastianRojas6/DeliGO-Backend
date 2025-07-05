use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::fmt;

use register_login::credential_validation::infrastructure::entity::sea_orm_active_enums::{
    MethodPaymentEnum as DbPaymentMethod,
    StatePaymentEnum as DbPaymentStatus,
};

// ---------------- Enums ----------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PaymentStatus {
    Pending,
    Paid,
    Cancelled,
}

impl fmt::Display for PaymentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TryFrom<DbPaymentStatus> for PaymentStatus {
    type Error = String;

    fn try_from(value: DbPaymentStatus) -> Result<Self, Self::Error> {
        match value {
            DbPaymentStatus::Pending => Ok(PaymentStatus::Pending),
            DbPaymentStatus::Paid => Ok(PaymentStatus::Paid),
            DbPaymentStatus::Cancelled => Ok(PaymentStatus::Cancelled),
        }
    }
}

impl From<PaymentStatus> for DbPaymentStatus {
    fn from(value: PaymentStatus) -> Self {
        match value {
            PaymentStatus::Pending => DbPaymentStatus::Pending,
            PaymentStatus::Paid => DbPaymentStatus::Paid,
            PaymentStatus::Cancelled => DbPaymentStatus::Cancelled,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PaymentMethod {
    Visa,
    MasterCard,
    Yape,
    Plin,
    Paypal,
}

impl fmt::Display for PaymentMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TryFrom<DbPaymentMethod> for PaymentMethod {
    type Error = String;

    fn try_from(value: DbPaymentMethod) -> Result<Self, Self::Error> {
        match value {
            DbPaymentMethod::Visa => Ok(PaymentMethod::Visa),
            DbPaymentMethod::MasterCard => Ok(PaymentMethod::MasterCard),
            DbPaymentMethod::Yape => Ok(PaymentMethod::Yape),
            DbPaymentMethod::Plin => Ok(PaymentMethod::Plin),
            DbPaymentMethod::Paypal => Ok(PaymentMethod::Paypal),
        }
    }
}

impl From<PaymentMethod> for DbPaymentMethod {
    fn from(value: PaymentMethod) -> Self {
        match value {
            PaymentMethod::Visa => DbPaymentMethod::Visa,
            PaymentMethod::MasterCard => DbPaymentMethod::MasterCard,
            PaymentMethod::Yape => DbPaymentMethod::Yape,
            PaymentMethod::Plin => DbPaymentMethod::Plin,
            PaymentMethod::Paypal => DbPaymentMethod::Paypal,
        }
    }
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id: Option<i32>,
    pub order_id: i32,
    pub total_amount: f64,
    pub payment_date: NaiveDateTime,
    pub payment_status: PaymentStatus,
    pub payment_method: PaymentMethod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceItem {
    pub product_name: String,
    pub quantity: i32,
    pub unit_price: f64,
    pub subtotal: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub order_id: i32,
    pub customer_name: String,
    pub delivery_address: String,
    pub payment_method: PaymentMethod,
    pub payment_date: NaiveDateTime,
    pub total_amount: f64,
    pub items: Vec<InvoiceItem>,
}
