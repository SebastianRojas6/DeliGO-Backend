#[derive(Debug, Clone)]
pub struct ProductInDto {
    pub id_product: Option<i32>,
    pub name: String,
    pub price: f32,
}

#[derive(Debug, Clone)]
pub struct ProductUpdateInDto {
    pub id_product: i32,
    pub name: Option<String>,
    pub price: Option<f32>,
}