use serde::Deserialize;
use shared::entity::sea_orm_active_enums::StateOrderEnum;

#[derive(Deserialize)]
pub struct OrderCreateDto {
    pub id_user: Option<i32>,
    pub id_delivery_man: Option<i32>,
    pub time: Option<String>,
    pub delivery_address: Option<String>,
    pub order_status: Option<String>,
}

#[derive(Deserialize)]
pub struct OrderUpdateDto {
    pub id_order: i32,
    pub id_user: Option<i32>,
    pub id_delivery_man: Option<i32>,
    pub time: Option<String>,
    pub delivery_address: Option<String>,
    pub order_status: Option<String>,
}

pub fn parse_order_status_to_enum(order_status: Option<String>) -> Option<StateOrderEnum> {
    order_status.map(|status| match status.as_str() {
        "pending" => StateOrderEnum::Pending,
        "preparing" => StateOrderEnum::Preparing,
        "ontheway" => StateOrderEnum::OnTheWay,
        "delivered" => StateOrderEnum::Delivered,
        "cancelled" => StateOrderEnum::Cancelled,
        _ => StateOrderEnum::Pending,
    })
}
