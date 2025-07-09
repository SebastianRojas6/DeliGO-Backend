use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct OrderEntity {
    pub id_order: i32,
    pub id_user: Option<i32>,
    pub id_delivery_man: Option<i32>,
    pub time: Option<NaiveDateTime>,
    pub delivery_address: Option<String>,
    pub order_status: Option<String>,
}
