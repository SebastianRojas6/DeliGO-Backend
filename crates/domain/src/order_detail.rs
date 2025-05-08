#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderDetailEntity {
    pub id_detail: i32,
    pub id_order: i32,
    pub id_product: i32,
    pub amount: i32,
}

impl OrderDetailEntity {
    pub fn new(
        id_detail: i32,
        id_order: i32,
        id_product: i32,
        amount: i32,
    ) -> Self {
        Self {
            id_detail,
            id_order,
            id_product,
            amount,
        }
    }
}