use domain::order::Model as OrderModel;


#[cfg(test)]
use mockall::{predicate::*, automock};
use std::error::Error;
use async_trait::async_trait;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait OrderFactRepositoryAbstract {
    async fn get_random_order_fact(&self) -> Result<OrderModel, Box<dyn Error>>;
    async fn get_all_order_facts(&self) -> Result<Vec<OrderModel>, Box<dyn Error>>;
}
