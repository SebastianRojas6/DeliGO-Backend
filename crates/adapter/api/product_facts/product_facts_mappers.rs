use application::DTOs::ProductDTOs::{CreateProductDTOs, UpdateProductDTOs};
use application::mappers::api_mapper::ApiMapper;
use domain::product::ProductEntity;
use crate::api::product_facts::product_facts_payloads::{InsertProductFactPayload, UpdateProductFactPayload};
use crate::api::product_facts::product_facts_presenters::ProductFactPresenter;

pub struct ProductFactPresenterMapper {}

impl ApiMapper<ProductEntity, ProductFactPresenter, InsertProductFactPayload, CreateProductDTOs> for ProductFactPresenterMapper {
    fn to_api(entity: ProductEntity) -> ProductFactPresenter {
        ProductFactPresenter {
            id_product: entity.id_product,
            name: entity.name,
            price: entity.price,
        }
    }

    fn to_dto(payload: InsertProductFactPayload) -> CreateProductDTOs {
        CreateProductDTOs {
            name: payload.name,
            price: payload.price,
        }
    }
}

pub struct ProductFactUpdateMapper {}

impl ApiMapper<ProductEntity, ProductFactPresenter, UpdateProductFactPayload, UpdateProductDTOs> for ProductFactUpdateMapper {
    fn to_api(entity: ProductEntity) -> ProductFactPresenter {
        ProductFactPresenter {
            id_product: entity.id_product,
            name: entity.name,
            price: entity.price,
        }
    }
    
    fn to_dto(payload: UpdateProductFactPayload) -> UpdateProductDTOs {
        UpdateProductDTOs {
            id_product: payload.id_product,
            name: payload.name,
            price: payload.price,
        }
    }
}