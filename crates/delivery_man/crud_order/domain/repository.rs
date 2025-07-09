use crate::crud_order::application::dtos::OrderCreateDto;
use crate::crud_order::application::dtos::OrderUpdateDto;
use crate::crud_order::domain::model::OrderEntity;
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait OrderAbstractRepository: Send + Sync {
    async fn get_all(&self) -> Result<Vec<OrderEntity>, Box<dyn Error>>;
    async fn get_by_id(&self, id: i32) -> Result<OrderEntity, Box<dyn Error>>;
    async fn create(&self, order: OrderCreateDto) -> Result<OrderEntity, Box<dyn Error>>;
    async fn update(&self, order: OrderUpdateDto) -> Result<OrderEntity, Box<dyn Error>>;
    async fn delete(&self, id: i32) -> Result<(), Box<dyn Error>>;
}
