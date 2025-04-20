use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

pub struct Model{

    pub id_order : i32,

    pub total_amount : i32,

    pub paymant_method : method,

    pub paymant_status : status,

}

pub enum method {
    Visa,
    MasterCard,
    Yape,
    Plin,
    Paypal,
}

pub enum status {
    Pending,
    Paid,
    Cancelled,
}