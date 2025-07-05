use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryManEntity {
    pub id: i32,
    pub name: String,
    pub phone: String,
    pub password: String,
    pub email: String,
    pub address: String,
    pub role: RolType,
    pub latitude: f64,
    pub longitude: f64,
}

pub struct OrderModel {
    pub id_user: i32,
    pub id_delivery_man: Option<i32>,
    pub time: String,
    pub delivery_address: String,
    pub order_status: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RolType {
    Administrator,
    Delivery,
    Customer,
}

