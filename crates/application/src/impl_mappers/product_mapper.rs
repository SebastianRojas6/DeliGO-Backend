use domain::product::ProductEntity;
use crate::DTOs::product::product_in_dto::ProductInDto;
use crate::DTOs::product::product_out_dto::ProductOutDto;
use crate::mappers::app_mapper::DTOMapper;

pub struct ProductMapper {}

impl DTOMapper<ProductEntity, ProductInDto, ProductOutDto> for ProductMapper {
    fn to_entity(dto: ProductInDto) -> ProductEntity {
        todo!()
    }


    fn to_dto(entity: ProductEntity) -> ProductOutDto {
        ProductOutDto {
            id_product: entity.id_product,
            name: entity.name,
            price: entity.price,
        }
    }

    fn to_dtos(entities: Vec<ProductEntity>) -> Vec<ProductOutDto> {
        todo!()
    }
}