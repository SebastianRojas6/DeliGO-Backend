use serde::{Serialize, Deserialize};

#[derive(Debug, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "order_status", rename_all = "lowercase")]
pub enum OrderStatus {
    Pending,
    Preparing,
    OnTheWay,
    Delivered,
    Cancelled,
}
