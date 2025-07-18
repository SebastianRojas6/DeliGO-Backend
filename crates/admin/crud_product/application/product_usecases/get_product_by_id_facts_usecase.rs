use std::sync::Arc;
use async_trait::async_trait;
use crate::crud_product::application::interface::{AbstractUseCase, ApiError};
use crate::crud_product::application::utils::ErrorHandlingUtils;
use crate::crud_product::domain::models::product::ProductEntity;
use crate::crud_product::domain::repository::ProductAbstractRepository;

pub struct GetProductByIdFactsUsecase {
    repository: Arc<dyn ProductAbstractRepository>,
    product_id: i32,
}

impl GetProductByIdFactsUsecase{
    pub fn new(repository: Arc<dyn ProductAbstractRepository>, product_id: i32) -> Self {
        GetProductByIdFactsUsecase { repository, product_id }
    }
}

#[async_trait(?Send)]
impl AbstractUseCase<ProductEntity> for GetProductByIdFactsUsecase {
    async fn execute(&self) -> Result<ProductEntity, ApiError> {
        let product = self.repository.get_product_by_id(self.product_id).await;
        match product {
            Ok(product) => Ok(product),
            Err(e) => Err(ErrorHandlingUtils::application_error("Error getting product by ID", Some(e))),
        }
    }
}