//use crate::domain::models::{product_model::Product, user_model::User};
use crate::domain::repository::UserRepository;
use shared::entity::{order, user};
use async_trait::async_trait;
use sea_orm::query::Condition;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use shared::connect_to_supabase;

pub struct UserQuery {
    pub db: DatabaseConnection,
}

impl UserQuery {
    pub async fn new() -> Result<Self, String> {
        let db = connect_to_supabase().await.map_err(|e| e.to_string())?;
        Ok(Self { db })
    }
}

#[async_trait]
impl UserRepository for UserQuery {
    async fn rating_delivery(&self, user_id: i32, delivery_id: i32) -> Result<(), String> {
        let order_exists = order::Entity::find()
            .filter(
                Condition::all()
                    .add(order::Column::IdUser.eq(user_id))
                    .add(order::Column::IdDeliveryMan.eq(delivery_id))
                    .add(order::Column::OrderStatus.eq("delivered")),
            )
            .one(&self.db)
            .await
            .map_err(|e| e.to_string())?;

        if order_exists.is_none() {
            return Err("No delivered orders found for this user/delivery combination".to_string());
        }

        let delivery_man = user::Entity::find_by_id(delivery_id)
            .one(&self.db)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Delivery man not found".to_string())?;

        let new_rating = match delivery_man.rating {
            Some(current_rating) => (current_rating + 5.0) / 2.0,
            None => 5.0,
        };

        let mut delivery_man: user::ActiveModel = delivery_man.into();
        delivery_man.rating = Set(Some(new_rating));
        delivery_man.update(&self.db).await.map_err(|e| e.to_string())?;

        Ok(())
    }
}
