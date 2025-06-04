use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductEntity {
    pub id_product: i32,
    pub name: String,
    pub price: f32,
}

impl ProductEntity {
    pub fn new(id_product: i32, name: String, price: f32) -> Self {
        ProductEntity {
            id_product,
            name,
            price,
        }
    }
}
