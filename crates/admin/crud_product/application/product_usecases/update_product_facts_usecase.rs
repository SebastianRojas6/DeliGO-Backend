use crate::crud_product::application::dto::product_dto::ProductUpdateDto;
use crate::crud_product::application::interface::{AbstractUseCase, ApiError};
use crate::crud_product::application::utils::ErrorHandlingUtils;
use crate::crud_product::domain::models::product::ProductEntity;
use crate::crud_product::domain::repository::ProductAbstractRepository;

pub struct UpdateProductFactsUseCase<'a> {
    pub product_repository: &'a dyn ProductAbstractRepository,
    pub product_update_dto: ProductUpdateDto,
}

impl <'a> UpdateProductFactsUseCase<'a> {
    pub fn new(
        product_repository: &'a dyn ProductAbstractRepository,
        product_update_dto: ProductUpdateDto,
    ) -> Self {
        UpdateProductFactsUseCase {
            product_repository,
            product_update_dto,
        }
    }
}

#[async_trait::async_trait(?Send)]
impl <'a> AbstractUseCase<ProductEntity> for UpdateProductFactsUseCase<'a> {
    async fn execute(&self) -> Result<ProductEntity, ApiError> {
        let updated_product = self.product_repository.update_product(self.product_update_dto.clone()).await;
        match updated_product {
            Ok(product) => Ok(product),
            Err(e) => Err(ErrorHandlingUtils::application_error("Error updating product facts", Some(e))),
        }
    }
}