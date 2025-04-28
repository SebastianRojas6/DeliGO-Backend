use bigdecimal::{FromPrimitive, ToPrimitive};
use sea_orm::prelude::Decimal;
use application::mappers::db_mapper::DbMapper;
use domain::product::{Model, ProductResponse};

pub struct ProductFactDbMapper {}

impl DbMapper<ProductResponse, Model> for ProductFactDbMapper {
    fn to_db(entity: ProductResponse) -> Model {
        Model {
            id_product: entity.id_product,
            name: entity.name,
            price: Decimal::from_f32(entity.price).unwrap_or_default(),
        }
    }

    fn to_entity(model: Model) -> ProductResponse {
        ProductResponse {
            id_product: model.id_product,
            name: model.name,
            price: model.price.to_f32().unwrap_or_default(),
        }
    }
}