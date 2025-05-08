#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeliveryManEntity {
    pub id_delivery_man: Option<i32>,
    pub name: String,
    pub phone: String,
}