use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductFactPayload {
    pub name: String,
    pub price: f32,
}