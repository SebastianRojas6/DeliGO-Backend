use bigdecimal::{FromPrimitive, ToPrimitive};
use sea_orm::prelude::Decimal;
use application::mappers::db_mapper::DbMapper;
use domain::product::ProductEntity;
use crate::spi::db::db_models::product::Model;

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