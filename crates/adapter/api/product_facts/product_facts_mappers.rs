use application::dtos::product::product_out_dto::ProductOutDto;
use application::dtos::product::product_in_dto::{ProductInDto, ProductUpdateInDto};
use application::mappers::api_mapper::{ApiInMapper, ApiOutMapper};
use crate::api::product_facts::product_facts_payloads::{InsertProductFactPayload, UpdateProductFactPayload};
use crate::api::product_facts::product_facts_presenters::ProductFactPresenter;

pub struct ProductFactPresenterMapper {}

impl ApiInMapper<InsertProductFactPayload, ProductInDto> for ProductFactPresenterMapper {
    fn to_api(entity: InsertProductFactPayload) -> ProductInDto {
        ProductInDto {
            id_product: None::<i32>,
            name: entity.name,
            price: entity.price,
        }
    }
}

impl ApiOutMapper<ProductOutDto, ProductFactPresenter> for ProductFactPresenterMapper {
    fn to_presenter(presenter: ProductOutDto) -> ProductFactPresenter {
        ProductFactPresenter {
            id_product: presenter.id_product,
            name: presenter.name,
            price: presenter.price,
        }
    }
}

impl ApiInMapper<UpdateProductFactPayload, ProductUpdateInDto> for ProductFactPresenterMapper {
    fn to_api(entity: UpdateProductFactPayload) -> ProductUpdateInDto {
        ProductUpdateInDto {
            id_product: entity.id_product,
            name: entity.name,
            price: entity.price,
        }
    }
}

