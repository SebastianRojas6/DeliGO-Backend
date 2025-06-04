use async_trait::async_trait;
use domain::error::ApiError;
use crate::DTOs::product::product_in_dto::ProductInDto;
use crate::DTOs::product::product_out_dto::ProductOutDto;
use crate::impl_mappers::product_mapper::ProductMapper;
use crate::mappers::app_mapper::DTOMapper;
use crate::repositories::product_fact_repository_abstract::ProductFactRepositoryAbstract;
use crate::usecases::interfaces::AbstractUseCase;
use crate::utils::error_handling_utils::ErrorHandlingUtils;

pub struct CreateProductFactUseCase<'a> {
    product: &'a ProductInDto,
    repository: &'a dyn ProductFactRepositoryAbstract,
}

impl<'a> CreateProductFactUseCase<'a> {
    pub fn new(
        product: &'a ProductInDto,
        repository: &'a dyn ProductFactRepositoryAbstract,
    ) -> Self {
        CreateProductFactUseCase { product, repository }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<ProductOutDto> for CreateProductFactUseCase<'a> {
    async fn execute(&self) -> Result<ProductOutDto, ApiError> {
        let result = self.repository.create_product_fact(self.product.clone()).await;
        match result {
            Ok(product) => Ok(ProductMapper::to_dto(product)),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot create product fact", Some(e))),
        }
    }
}