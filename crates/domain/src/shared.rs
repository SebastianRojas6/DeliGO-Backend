use sea_orm::{DeriveActiveEnum, EnumIter}; 
use serde::{Deserialize, Serialize};
use strum_macros::{EnumString, Display};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, Display, DeriveActiveEnum, EnumIter)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "order_status")]
pub enum OrderStatus {
    #[sea_orm(string_value = "pending")]
    Pending,

    #[sea_orm(string_value = "preparing")]
    Preparing,

    #[sea_orm(string_value = "on_the_way")]
    OnTheWay,

    #[sea_orm(string_value = "delivered")]
    Delivered,

    #[sea_orm(string_value = "cancelled")]
    Cancelled,
}
