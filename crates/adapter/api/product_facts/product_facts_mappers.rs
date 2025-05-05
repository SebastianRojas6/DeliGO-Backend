use application::mappers::api_mapper::ApiMapper;
use domain::product::ProductEntity;
use crate::api::product_facts::product_facts_payloads::InsertProductFactPayload;
use crate::api::product_facts::product_facts_presenters::ProductFactPresenter;

pub struct ProductFactPresenterMapper {}

impl ApiMapper<ProductEntity, ProductFactPresenter, InsertProductFactPayload> for ProductFactPresenterMapper {
    fn to_api(entity: ProductEntity) -> ProductFactPresenter {
        ProductFactPresenter {
            id_product: entity.id_product,
            name: entity.name,
            price: entity.price,
        }
    }

    fn to_entity(_payload: InsertProductFactPayload) -> ProductEntity {
        panic!("not implemented")
    }
}