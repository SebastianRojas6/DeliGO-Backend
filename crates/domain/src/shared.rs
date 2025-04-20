use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "order_status", rename_all = "snake_case")]
pub enum OrderStatus {
    Pending,
    Preparing,
    OnTheWay,
    Delivered,
    Cancelled,
}
