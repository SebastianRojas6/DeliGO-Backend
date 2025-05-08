use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductFactPresenter {
    pub id_product: i32,
    pub name: String,
    pub price: f32,
}