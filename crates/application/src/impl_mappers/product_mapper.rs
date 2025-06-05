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

    fn to_dtos(_entities: Vec<ProductEntity>) -> Vec<ProductOutDto> {
        todo!()
    }
}
