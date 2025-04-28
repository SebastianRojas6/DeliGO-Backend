use application::mappers::api_mapper::ApiMapper;
use domain::product::ProductResponse;
use crate::api::product_facts::product_facts_payloads::ProductFactPayload;
use crate::api::product_facts::product_facts_presenters::ProductFactPresenter;

pub struct ProductFactPresenterMapper {}

impl ApiMapper<ProductResponse, ProductFactPresenter, ProductFactPayload> for ProductFactPresenterMapper {
    fn to_api(entity: ProductResponse) -> ProductFactPresenter {
        ProductFactPresenter {
            id_product: entity.id_product,
            name: entity.name,
            price: entity.price,
        }
    }

    fn to_entity(_payload: ProductFactPayload) -> ProductResponse {
        panic!("not implemented")
    }
}