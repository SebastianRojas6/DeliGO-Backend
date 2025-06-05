use domain::order::OrderEntity;


#[cfg(test)]
use mockall::{predicate::*, automock};
use std::error::Error;
use async_trait::async_trait;
use crate::dtos::order_dto::{CreateOrderDTOs, UpdateOrderDTOs};

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait OrderFactRepositoryAbstract {
    async fn get_all_order_facts(&self) -> Result<Vec<OrderEntity>, Box<dyn Error>>;
    async fn get_one_order_by_id_fact(&self, fact_id: i32) -> Result<OrderEntity, Box<dyn Error>>;
    async fn get_all_order_facts_by_user_id(&self, user_id: i32) -> Result<Vec<OrderEntity>, Box<dyn Error>>;
    async fn get_all_order_facts_by_delivery_id(&self, delivery_id: i32) -> Result<Vec<OrderEntity>, Box<dyn Error>>;
    async fn get_all_order_with_details_by_user_id(&self, user_id: i32) -> Result<Vec<OrderEntity>, Box<dyn Error>>;
    async fn create_order_fact(&self, order_fact: CreateOrderDTOs) -> Result<OrderEntity, Box<dyn Error>>;
    async fn update_order_fact(&self, order_fact: UpdateOrderDTOs) -> Result<OrderEntity, Box<dyn Error>>;
    async fn delete_order_fact_by_id(&self, order_fact_id: i32) -> Result<bool, Box<dyn Error>>;

}
