use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct DeliveryLocationEntity {
    pub id_location: i32,
    pub id_delivery_man: Option<i32>,
    pub latitude: f32,
    pub longitude: f32,
    pub time_delivery_man: Option<NaiveDateTime>,
}