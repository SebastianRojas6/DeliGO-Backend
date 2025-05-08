use serde::{Deserialize, Serialize};
use strum_macros::{EnumString, Display};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumString, Display)]
pub enum OrderStatus {
    Pending,

    Preparing,

    OnTheWay,

    Delivered,

    Cancelled,
}
