use domain::product::ProductEntity;

#[cfg(test)]
use mockall::{predicate::*, *};
use std::error::Error;
use async_trait::async_trait;
use crate::DTOs::product::product_in_dto::{ProductInDto, ProductUpdateInDto};

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait ProductFactRepositoryAbstract {
    async fn get_one_product_by_id_fact(&self, fact_id : i32) -> Result<ProductEntity, Box<dyn Error>>;
    async fn get_all_product_facts(&self) -> Result<Vec<ProductEntity>, Box<dyn Error>>;
    async fn delete_product_by_id_fact(&self, fact_id: i32) -> Result<(), Box<dyn Error>>;
    async fn create_product_fact(&self, product: ProductInDto) -> Result<ProductEntity, Box<dyn Error>>;
    async fn update_product_fact(&self, product: ProductUpdateInDto) -> Result<ProductEntity, Box<dyn Error>>;
}