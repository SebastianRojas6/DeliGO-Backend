use super::models::product_model::Product;
use async_trait::async_trait;

#[async_trait]
pub trait ProductRepository: Send + Sync {
    async fn get_all(&self) -> Result<Vec<Product>, String>;
    async fn get_by_purchase_for_user(&self, user_id: i32, product_id: i32) -> Result<Vec<Product>, String>;
    async fn get_selected_products(&self, user_id: i32) -> Result<Vec<Product>, String>;
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn rating_delivery(&self, user_id: i32, delivery_id: i32) -> Result<(), String>;
}
