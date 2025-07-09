// use_cases/order_use_case.rs
use crate::crud_order::application::dtos::{OrderCreateDto, OrderUpdateDto};
use crate::crud_order::domain::model::OrderEntity;
use crate::crud_order::domain::repository::OrderAbstractRepository;
use std::sync::Arc;

pub struct OrderUseCase {
    pub repo: Arc<dyn OrderAbstractRepository>,
}

impl OrderUseCase {
    pub fn new(repo: Arc<dyn OrderAbstractRepository>) -> Self {
        Self { repo }
    }

    pub async fn list(&self) -> Result<Vec<OrderEntity>, Box<dyn std::error::Error>> {
        self.repo.get_all().await
    }

    pub async fn get(&self, id: i32) -> Result<OrderEntity, Box<dyn std::error::Error>> {
        self.repo.get_by_id(id).await
    }

    pub async fn create(&self, dto: OrderCreateDto) -> Result<OrderEntity, Box<dyn std::error::Error>> {
        self.repo.create(dto).await
    }

    pub async fn update(&self, dto: OrderUpdateDto) -> Result<OrderEntity, Box<dyn std::error::Error>> {
        self.repo.update(dto).await
    }

    pub async fn delete(&self, id: i32) -> Result<(), Box<dyn std::error::Error>> {
        self.repo.delete(id).await
    }
}
