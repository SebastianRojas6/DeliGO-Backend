use async_trait::async_trait;
use domain::error::ApiError;
use crate::repositories::product_fact_repository_abstract::ProductFactRepositoryAbstract;
use crate::usecases::interfaces::AbstractUseCase;
use domain::product::{ProductEntity};
use crate::DTOs::product::product_out_dto::ProductOutDto;
use crate::impl_mappers::product_mapper::ProductMapper;
use crate::mappers::app_mapper::DTOMapper;

pub struct GetOneProductByIdUseCase<'a> {
    product_fact_id: &'a i32,
    repository: &'a dyn ProductFactRepositoryAbstract,
}

impl<'a> GetOneProductByIdUseCase<'a> {
    pub fn new(product_fact_id: &'a i32, repository: &'a dyn ProductFactRepositoryAbstract) -> Self {
        GetOneProductByIdUseCase { product_fact_id, repository }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<ProductOutDto> for GetOneProductByIdUseCase<'a> {
    async fn execute(&self) -> Result<ProductOutDto, ApiError> {
        let product = self.repository.get_one_product_by_id_fact(*self.product_fact_id).await;
        match product {
            Ok(product) => Ok(ProductMapper::to_dto(product)),
            Err(e) => Err(crate::utils::error_handling_utils::ErrorHandlingUtils::application_error("Cannot get one product", Some(e))),
        }
    }
}

