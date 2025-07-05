use std::error::Error;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use crate::crud_delivery_man::application::dto::delivery_man_dto::DeliveryManUpdateDto;
use crate::crud_delivery_man::domain::model::DeliveryManEntity;
use crate::crud_delivery_man::domain::repository::DeliveryManAbstractRepository;
use crate::crud_delivery_man::infrastructure::spi::entity::user::{Entity, Model, self};
use crate::crud_delivery_man::infrastructure::spi::entity::order::Entity as OrderEntity;
use crate::crud_delivery_man::infrastructure::spi::db_mapper::DBMapper;
use crate::crud_delivery_man::infrastructure::spi::entity::order;
use crate::crud_delivery_man::infrastructure::spi::entity::sea_orm_active_enums::{map_roltype_from_entity, roltype_from_i32};

pub struct DbDeliveryFactsRepository {
    connection: DatabaseConnection,
}

impl DbDeliveryFactsRepository {
    pub fn new(connection: DatabaseConnection) -> Self {
        DbDeliveryFactsRepository { connection }
    }

}

#[async_trait::async_trait(?Send)]
impl DeliveryManAbstractRepository for DbDeliveryFactsRepository {
    async fn get_all_delivery_mans(&self) -> Result<Vec<DeliveryManEntity>, Box<dyn Error>> {
        let delivery_mans = Entity::find().all(&self.connection).await.map_err(|e| {
            Box::new(e) as Box<dyn Error>
        })?;
        
        let delivery_mans_response = delivery_mans.into_iter()
            .map(|delivery_man| DBMapper::to_entity(delivery_man))
            .collect();
        Ok(delivery_mans_response)
    }

    async fn get_delivery_info_by_id(&self, id: i32) -> Result<DeliveryManEntity, Box<dyn Error>> {
        let delivery_man = Entity::find_by_id(id)
            .one(&self.connection)
            .await
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;
        match delivery_man {
            Some(delivery) => Ok(DBMapper::to_entity(delivery)),
            None => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Delivery not found",
            )) as Box<dyn Error>),
        }

    }

    async fn asign_delivery_to_order(&self, id_delivery: i32, id_order: i32) -> Result<(), Box<dyn Error>> {
        let order = OrderEntity::find_by_id(id_order)
            .one(&self.connection)
            .await
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;
        match order {
            Some(order) => {
                let mut order_active: order::ActiveModel = order.into();

                order_active.id_delivery_man = Set(Some(id_delivery));
                order_active.update(&self.connection).await.map_err(|e| Box::new(e) as Box<dyn Error>)?;
                Ok(())
            },
            None => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Order not found",
            )) as Box<dyn Error>),
        }
        
    }

    async fn update_delivery_man(&self, delivery_man_update_dto: DeliveryManUpdateDto) -> Result<DeliveryManEntity, Box<dyn Error>> {
        let delivery_man_option = Entity::find_by_id(delivery_man_update_dto.id)
            .one(&self.connection)
            .await
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        match delivery_man_option {
            Some(delivery_man) => {
                let mut active_model: user::ActiveModel = delivery_man.into();

                if let Some(name) = delivery_man_update_dto.name {
                    active_model.name = Set(Some(name));
                }
                if let Some(phone) = delivery_man_update_dto.phone {
                    active_model.phone = Set(Some(phone));
                }
                if let Some(address) = delivery_man_update_dto.address {
                    active_model.address = Set(Some(address));
                }
                if let Some(role_value) = delivery_man_update_dto.role {
                    if let Some(role_enum) = roltype_from_i32(role_value) {
                        active_model.rol = Set(Some(role_enum));
                    }
                }
                if let Some(latitude) = delivery_man_update_dto.latitude {
                    active_model.latitud = Set(Some(latitude.to_string()));
                }
                if let Some(longitude) = delivery_man_update_dto.longitude {
                    active_model.latitud = Set(Some(longitude.to_string()));
                }

                let updated = active_model
                    .update(&self.connection)
                    .await
                    .map_err(|e| Box::new(e) as Box<dyn Error>)?;
                Ok(DBMapper::to_entity(updated))
            }
            None => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Delivery man not found",
            )) as Box<dyn Error>),
        }
    }}