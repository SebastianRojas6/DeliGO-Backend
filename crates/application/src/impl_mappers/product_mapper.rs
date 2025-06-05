use domain::product::ProductEntity;
use crate::dtos::product::product_out_dto::ProductOutDto;
use crate::mappers::app_mapper::{DTOMapper};

pub struct ProductMapper {}

impl DTOMapper<ProductEntity, ProductOutDto> for ProductMapper {

    fn to_dto(entity: ProductEntity) -> ProductOutDto {
        ProductOutDto {
            id_product: entity.id_product,
            name: entity.name,
            price: entity.price,
        }
    }

    fn to_dtos(entities: Vec<ProductEntity>) -> Vec<ProductOutDto> {
        entities.into_iter().map(ProductMapper::to_dto).collect()
    }
}
