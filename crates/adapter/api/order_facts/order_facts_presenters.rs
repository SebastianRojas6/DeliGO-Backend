use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderFactRequest {
    pub id_order: i32,
    pub id_product: i32,
    pub quantity: i32,
}