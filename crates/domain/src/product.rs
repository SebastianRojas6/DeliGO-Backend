use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Product{
    pub id_product : i32,
    pub name : String,
    pub price : BigDecimal,

}

#[derive(Debug, Deserialize)]
pub struct ProductRequest {
    pub name: String,
    pub price: f32,
}

#[derive(Debug, Serialize)]
pub struct ProductResponse {
    pub id_product: i32,
    pub name: String,
    pub price: f32,
}
