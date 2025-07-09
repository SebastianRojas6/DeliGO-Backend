// infrastructure/order_repository_sea_orm.rs
use crate::crud_order::application::dtos::{parse_order_status_to_enum, OrderCreateDto, OrderUpdateDto};
use crate::crud_order::domain::model::OrderEntity;
use crate::crud_order::domain::repository::OrderAbstractRepository;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use sea_orm::*;
use shared::entity::order;
use shared::entity::sea_orm_active_enums::StateOrderEnum;
use std::error::Error;

pub struct OrderRepositorySeaOrm {
    pub db: DatabaseConnection,
}

impl OrderRepositorySeaOrm {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl OrderAbstractRepository for OrderRepositorySeaOrm {
    async fn get_all(&self) -> Result<Vec<OrderEntity>, Box<dyn Error>> {
        let orders = order::Entity::find().all(&self.db).await?;
        Ok(orders.into_iter().map(|o| o.into()).collect())
    }

    async fn get_by_id(&self, id: i32) -> Result<OrderEntity, Box<dyn Error>> {
        let order = order::Entity::find_by_id(id).one(&self.db).await?.ok_or("Order not found")?;
        Ok(order.into())
    }

    async fn create(&self, dto: OrderCreateDto) -> Result<OrderEntity, Box<dyn Error>> {
        let new = order::ActiveModel {
            id_order: NotSet,
            id_user: Set(dto.id_user),
            id_delivery_man: Set(dto.id_delivery_man),
            time: Set(dto.time.map(|s| NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").unwrap())),
            delivery_address: Set(dto.delivery_address),
            order_status: Set(parse_order_status_to_enum(dto.order_status)),
        };

        let saved = new.insert(&self.db).await?;
        Ok(saved.into())
    }

    async fn update(&self, dto: OrderUpdateDto) -> Result<OrderEntity, Box<dyn Error>> {
        let mut existing = order::Entity::find_by_id(dto.id_order).one(&self.db).await?.ok_or("Order not found")?.into_active_model();

        existing.id_user = Set(dto.id_user);
        existing.id_delivery_man = Set(dto.id_delivery_man);
        existing.time = Set(dto.time.map(|s| NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").unwrap()));
        existing.delivery_address = Set(dto.delivery_address);
        existing.order_status = Set(parse_order_status_to_enum(dto.order_status));

        let updated = existing.update(&self.db).await?;
        Ok(updated.into())
    }

    async fn delete(&self, id: i32) -> Result<(), Box<dyn Error>> {
        order::Entity::delete_by_id(id).exec(&self.db).await?;
        Ok(())
    }
}

// conversión Entity -> Domain
impl From<order::Model> for OrderEntity {
    fn from(m: order::Model) -> Self {
        Self {
            id_order: m.id_order,
            id_user: m.id_user,
            id_delivery_man: m.id_delivery_man,
            time: m.time,
            delivery_address: m.delivery_address,
            order_status: m.order_status.map(|status| match status {
                StateOrderEnum::Pending => "Pending".to_string(),
                StateOrderEnum::Preparing => "Preparing".to_string(),
                StateOrderEnum::OnTheWay => "OnTheWay".to_string(),
                StateOrderEnum::Delivered => "Delivered".to_string(),
                StateOrderEnum::Cancelled => "Cancelled".to_string(),
            }),
        }
    }
}
