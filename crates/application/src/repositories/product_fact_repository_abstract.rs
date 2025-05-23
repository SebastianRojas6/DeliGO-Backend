use domain::product::ProductEntity;

#[cfg(test)]
use mockall::{predicate::*, *};
use std::error::Error;
use async_trait::async_trait;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait ProductFactRepositoryAbstract {
    async fn get_one_product_by_id_fact(&self, fact_id : i32) -> Result<ProductEntity, Box<dyn Error>>;
    async fn get_all_product_facts(&self) -> Result<Vec<ProductEntity>, Box<dyn Error>>;
    async fn delete_product_by_id_fact(&self, fact_id: i32) -> Result<(), Box<dyn Error>>;
}