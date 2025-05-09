use serde::{ Serialize};

#[derive(Debug, Serialize)]
pub struct ProductFactPresenter {
    #[serde(rename = "identifier")]
    pub id_product: i32,
    pub name: String,
    pub price: f32,
}