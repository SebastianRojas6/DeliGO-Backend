use domain::product::{Model as ProductModel, ProductResponse};

#[cfg(test)]
use mockall::{predicate::*, *};
use std::error::Error;
use async_trait::async_trait;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait ProductFactRepositoryAbstract {
    async fn get_one_product_by_id_fact(&self, fact_id : i32) -> Result<ProductResponse, Box<dyn Error>>;
    async fn get_all_product_facts(&self) -> Result<Vec<ProductResponse>, Box<dyn Error>>;
}