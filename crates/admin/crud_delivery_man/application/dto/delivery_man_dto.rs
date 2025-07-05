use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryManUpdateDto {
    pub id: i32,           
    pub name: Option<String>,
    pub phone: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub role: Option<i32>, // Assuming role is represented as an integer
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}