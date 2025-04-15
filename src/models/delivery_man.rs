use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct DeliveryMan {

    id_delivery_man : i32,
    name : String,
    phone : String

}