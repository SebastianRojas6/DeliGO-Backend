use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateProductDTOs {
    pub name: String,
    pub price: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateProductDTOs {
    pub id_product: i32,
    pub name: Option<String>,
    pub price: Option<f32>,
}