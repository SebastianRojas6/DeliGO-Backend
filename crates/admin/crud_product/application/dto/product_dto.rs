#[derive(Debug, Clone)]
pub struct ProductCreateDto {
    pub name: String,
    pub price: f32,
}

#[derive(Debug, Clone)]
pub struct ProductUpdateDto {
    pub id_product: i32,
    pub name: Option<String>,
    pub price: Option<f32>,
}