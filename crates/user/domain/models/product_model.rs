#[derive(Debug, Clone, serde::Serialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub price: String,
}
