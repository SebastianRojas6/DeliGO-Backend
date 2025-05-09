use async_trait::async_trait;
use domain::product::ProductEntity;
use crate::usecases::interfaces::AbstractUseCase;

pub struct UpdateProductFactUseCase<'a> {
    product: &'a crate::DTOs::ProductDTOs::UpdateProductDTOs,
    repository: &'a dyn crate::repositories::product_fact_repository_abstract::ProductFactRepositoryAbstract,
}

impl<'a> UpdateProductFactUseCase<'a> {
    pub fn new(
        product: &'a crate::DTOs::ProductDTOs::UpdateProductDTOs,
        repository: &'a dyn crate::repositories::product_fact_repository_abstract::ProductFactRepositoryAbstract,
    ) -> Self {
        UpdateProductFactUseCase { product, repository }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<ProductEntity> for UpdateProductFactUseCase<'a> {
    async fn execute(&self) -> Result<ProductEntity, domain::error::ApiError> {
        let result = self.repository.update_product_fact(self.product.clone()).await;
        match result {
            Ok(product) => Ok(product),
            Err(e) => Err(crate::utils::error_handling_utils::ErrorHandlingUtils::application_error("Cannot update product fact", Some(e))),
        }
    }
}