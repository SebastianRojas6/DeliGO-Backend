use std::error::Error;
use crate::crud_delivery_man::domain::model::DeliveryManEntity;

use async_trait::async_trait;
use crate::crud_delivery_man::application::dto::delivery_man_dto::DeliveryManUpdateDto;

#[async_trait(?Send)]
pub trait  DeliveryManAbstractRepository: Send + Sync {
    async fn get_all_delivery_mans(&self) -> Result<Vec<DeliveryManEntity>, Box<dyn Error>>;
    async fn get_delivery_info_by_id(&self, id: i32) -> Result<DeliveryManEntity, Box<dyn Error>>;
    async fn asign_delivery_to_order(&self, id_order: i32, id_delivery: i32) -> Result<(), Box<dyn Error>>;
    async fn update_delivery_man(&self, delivery_man_update_dto: DeliveryManUpdateDto) -> Result<DeliveryManEntity, Box<dyn Error>>;
}