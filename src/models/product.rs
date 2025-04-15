use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Product{

    pub id_product : i8,
    pub name : String,
    pub price : f32,

}