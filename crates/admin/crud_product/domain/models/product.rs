use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductEntity {
    pub id_product: i32,
    pub name: String,
    pub price: f32,
}