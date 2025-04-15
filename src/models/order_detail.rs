use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct OrderDetails{

    pub id_detail : i32,
    pub id_order : i32,
    pub id_product : i32,
    pub amount : i32

}