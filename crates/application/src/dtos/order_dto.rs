#[derive(Debug, Clone)]
pub struct CreateOrderDTOs {
    pub id_product: i32,
    pub quantity: i32,
}

#[derive(Debug, Clone)]
pub struct UpdateOrderDTOs {
    pub id_order: i32,
    pub id_product: Option<i32>,
    pub quantity: Option<i32>,
}