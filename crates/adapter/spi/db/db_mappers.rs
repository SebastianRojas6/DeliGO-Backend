use bigdecimal::{FromPrimitive, ToPrimitive};
use sea_orm::prelude::Decimal;
use application::mappers::db_mapper::DbMapper;
use domain::product::ProductEntity;
use domain::user::UserEntity;
use crate::spi::db::db_models::product::Model;
use crate::spi::db::db_models::user;

pub struct ProductFactDbMapper {}

impl DbMapper<ProductEntity, Model> for ProductFactDbMapper {
    fn to_db(entity: ProductEntity) -> Model {
        Model {
            id_product: entity.id_product,
            name: entity.name,
            price: Decimal::from_f32(entity.price).unwrap_or_default(),
        }
    }

    fn to_entity(model: Model) -> ProductEntity {
        ProductEntity {
            id_product: model.id_product,
            name: model.name,
            price: model.price.to_f32().unwrap_or_default(),
        }
    }
}

pub struct UserFactDbMapper {}

impl DbMapper<UserEntity, user::Model> for UserFactDbMapper {
    fn to_db(entity: UserEntity) -> user::Model {
        user::Model {
            id_user: entity.id_user,
            name: entity.name,
            phone: entity.phone,
            address: entity.address,
        }
    }

    fn to_entity(model: user::Model) -> UserEntity {
        UserEntity {
            id_user: model.id_user,
            name: model.name,
            phone: model.phone,
            address: model.address,       
        }
    }
}