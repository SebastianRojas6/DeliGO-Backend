use crate::crud_product::application::dto::product_dto::ProductCreateDto;
use crate::crud_product::application::interface::ApiError;
use crate::crud_product::application::utils::ErrorHandlingUtils;
use crate::crud_product::domain::models::product::ProductEntity;
use crate::crud_product::domain::repository::ProductAbstractRepository;

pub struct CreateProductFactsUseCase<'a> {
    repository: &'a dyn ProductAbstractRepository,
    product_create_dto: ProductCreateDto
}

impl<'a> CreateProductFactsUseCase<'a> {
    pub fn new(repository: &'a dyn ProductAbstractRepository, product_create_dto: ProductCreateDto) -> Self {
        CreateProductFactsUseCase {
            repository,
            product_create_dto,
        }
    }
}

#[async_trait::async_trait(?Send)]
impl<'a> crate::crud_product::application::interface::AbstractUseCase<ProductEntity> for CreateProductFactsUseCase<'a> {
    async fn execute(&self) -> Result<ProductEntity, ApiError> {
        let product = self.repository.create_product(self.product_create_dto.clone()).await;
        match product {
            Ok(product) => Ok(product),
            Err(e) => Err(ErrorHandlingUtils::application_error("Error creating product", Some(e))),
        }
    }
}