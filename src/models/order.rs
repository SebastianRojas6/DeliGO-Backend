use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::models::shared::OrderStatus;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Order {
    pub id_order: i32,
    pub id_user: i32,
    pub id_delivery_man: i32,
    pub time: NaiveDateTime,
    pub state: OrderStatus,
}

#[derive(Debug, Deserialize)]
pub struct OrderRequest {
    pub user_id: i32,
    pub items: Vec<i32>,
    pub delivery_address: String,
}

#[derive(Debug, Serialize)]
pub struct OrderResponse {
    pub id: i32,
    pub status: OrderStatus,
    pub estimated_time: String,
}
