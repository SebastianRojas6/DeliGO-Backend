use crate::shared::OrderStatus;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OrderEntity {
    pub id_order: i32,
    pub id_user: i32,
    pub id_delivery_man: Option<i32>,
    pub time: chrono::DateTime<chrono::FixedOffset>,
    pub state: OrderStatus,
    pub delivery_address: String,
}