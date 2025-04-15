use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum OrderStatus {
    Pending,
    Preparing,
    OnTheWay,
    Delivered,
    Cancelled,
}
