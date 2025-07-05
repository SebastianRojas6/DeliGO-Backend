use std::error::Error;
use async_trait::async_trait;
use crate::crud_product::application::dto::product_dto::{ProductCreateDto, ProductUpdateDto};
use crate::crud_product::domain::models::product::ProductEntity;

#[async_trait(?Send)]
pub trait ProductAbstractRepository: Send + Sync {
    async fn get_all_products(&self) -> Result<Vec<ProductEntity>, Box<dyn Error>>;
    async fn get_product_by_id(&self, id: i32) -> Result<ProductEntity, Box<dyn Error>>;
    async fn create_product(&self, product: ProductCreateDto) -> Result<ProductEntity, Box<dyn Error>>;
    async fn update_product(&self, product: ProductUpdateDto) -> Result<ProductEntity, Box<dyn Error>>;
    async fn delete_product(&self, id: i32) -> Result<(), Box<dyn Error>>;
}