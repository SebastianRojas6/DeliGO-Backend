use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct DeliveryLocation{

    pub id_location : i32,
    pub id_delivery_man : i32,
    pub latitude : f32,
    pub longitude : f32,
    pub time_delivery_man : NaiveDateTime,

}