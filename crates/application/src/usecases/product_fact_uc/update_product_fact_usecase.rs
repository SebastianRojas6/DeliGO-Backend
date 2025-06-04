use async_trait::async_trait;
use domain::product::ProductEntity;
use crate::DTOs::product::product_in_dto::ProductUpdateInDto;
use crate::DTOs::product::product_out_dto::ProductOutDto;
use crate::impl_mappers::product_mapper::ProductMapper;
use crate::mappers::app_mapper::DTOMapper;
use crate::repositories::product_fact_repository_abstract::ProductFactRepositoryAbstract;
use crate::usecases::interfaces::AbstractUseCase;

pub struct UpdateProductFactUseCase<'a> {
    product: &'a ProductUpdateInDto,
    repository: &'a dyn ProductFactRepositoryAbstract,
}

impl<'a> UpdateProductFactUseCase<'a> {
    pub fn new(
        product: &'a ProductUpdateInDto,
        repository: &'a dyn ProductFactRepositoryAbstract,
    ) -> Self {
        UpdateProductFactUseCase { product, repository }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<ProductOutDto> for UpdateProductFactUseCase<'a> {
    async fn execute(&self) -> Result<ProductOutDto, domain::error::ApiError> {
        let result = self.repository.update_product_fact(self.product.clone()).await;
        match result {
            Ok(product) => Ok(ProductMapper::to_dto(product)),
            Err(e) => Err(crate::utils::error_handling_utils::ErrorHandlingUtils::application_error("Cannot update product fact", Some(e))),
        }
    }
}