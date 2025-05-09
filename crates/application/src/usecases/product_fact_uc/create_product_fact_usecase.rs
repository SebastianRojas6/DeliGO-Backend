use domain::product::ProductEntity;
use crate::usecases::interfaces::AbstractUseCase;

pub struct CreateProductFactUseCase<'a> {
    product: &'a crate::DTOs::ProductDTOs::CreateProductDTOs,
    repository: &'a dyn crate::repositories::product_fact_repository_abstract::ProductFactRepositoryAbstract,
}

impl<'a> CreateProductFactUseCase<'a> {
    pub fn new(
        product: &'a crate::DTOs::ProductDTOs::CreateProductDTOs,
        repository: &'a dyn crate::repositories::product_fact_repository_abstract::ProductFactRepositoryAbstract,
    ) -> Self {
        CreateProductFactUseCase { product, repository }
    }
}

#[async_trait::async_trait(?Send)]
impl<'a> AbstractUseCase<ProductEntity> for CreateProductFactUseCase<'a> {
    async fn execute(&self) -> Result<ProductEntity, domain::error::ApiError> {
        let result = self.repository.create_product_fact(self.product.clone()).await;
        match result {
            Ok(product) => Ok(product),
            Err(e) => Err(crate::utils::error_handling_utils::ErrorHandlingUtils::application_error("Cannot create product fact", Some(e))),
        }
    }
}