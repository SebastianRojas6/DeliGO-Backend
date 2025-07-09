//use crate::domain::models::{product_model::Product, user_model::User};
use crate::domain::repository::UserRepository;
use async_trait::async_trait;
use sea_orm::query::Condition;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set};
use shared::connect_to_supabase;
use shared::entity::sea_orm_active_enums::StateOrderEnum;
use shared::entity::{order, user};

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

    async fn change_order_status(&self, id: &str, estado: &str) -> Result<String, String> {
        // Parsear el ID a i32
        let order_id = id.parse::<i32>().map_err(|_| "ID de orden inválido".to_string())?;

        // Convertir el string de estado al enum correspondiente
        let new_status = match estado.to_lowercase().as_str() {
            "pending" => StateOrderEnum::Pending,
            "preparing" => StateOrderEnum::Preparing,
            "ontheway" => StateOrderEnum::OnTheWay,
            "delivered" => StateOrderEnum::Delivered,
            "cancelled" => StateOrderEnum::Cancelled,
            _ => return Err("Estado de orden no válido".to_string()),
        };

        // Buscar la orden en la base de datos
        let order = order::Entity::find_by_id(order_id).one(&self.db).await.map_err(|e| e.to_string())?;

        let mut order = match order {
            Some(o) => o.into_active_model(),
            None => return Err("Orden no encontrada".to_string()),
        };

        // Actualizar el estado
        order.order_status = Set(Some(new_status.clone()));

        // Guardar los cambios
        let updated_order = order.update(&self.db).await.map_err(|e| e.to_string())?;

        // Devolver el nuevo estado como string
        match updated_order.order_status {
            Some(status) => Ok(match status {
                StateOrderEnum::Pending => "Pending".to_string(),
                StateOrderEnum::Preparing => "Preparing".to_string(),
                StateOrderEnum::OnTheWay => "OnTheWay".to_string(),
                StateOrderEnum::Delivered => "Delivered".to_string(),
                StateOrderEnum::Cancelled => "Cancelled".to_string(),
            }),
            None => Err("El estado de la orden es nulo".to_string()),
        }
    }
}
