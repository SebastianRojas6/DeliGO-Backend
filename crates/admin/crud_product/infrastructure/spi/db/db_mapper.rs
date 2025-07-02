use crate::crud_product::domain::models::product::ProductEntity;
use crate::crud_product::infrastructure::spi::db::product_entity::Model;

pub struct DBMapper;

impl DBMapper {
    pub fn to_entity(product: Model) -> ProductEntity {
        ProductEntity {
            id_product: product.id_product,
            name: product.name,
            price: f32::try_from(product.price).unwrap_or(0.0),
        }
    }
}