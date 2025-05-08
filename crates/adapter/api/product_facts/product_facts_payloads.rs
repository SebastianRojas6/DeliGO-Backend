use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct InsertProductFactPayload {
    pub name: String,
    pub price: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateProductFactPayload {
    pub id_product: i32,
    pub name: Option<String>,
    pub price: Option<f32>,
}