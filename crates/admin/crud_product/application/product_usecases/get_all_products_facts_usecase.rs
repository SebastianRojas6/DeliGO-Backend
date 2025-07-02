use async_trait::async_trait;
use crate::crud_product::application::interface::{AbstractUseCase, ApiError};
use crate::crud_product::application::utils::ErrorHandlingUtils;
use crate::crud_product::domain::models::product::ProductEntity;
use crate::crud_product::domain::repository::ProductAbstractRepository;

pub struct GetAllProductsFactsUseCase<'a> {
    repository: &'a dyn ProductAbstractRepository,
}

impl <'a> GetAllProductsFactsUseCase<'a> {
    pub fn new(repository: &'a dyn ProductAbstractRepository) -> Self {
        GetAllProductsFactsUseCase { repository }
    }
}

#[async_trait(?Send)]
impl <'a> AbstractUseCase<Vec<ProductEntity>> for GetAllProductsFactsUseCase<'a> {
    async fn execute(&self) -> Result<Vec<ProductEntity>, ApiError> {
        let products = self.repository.get_all_products().await;
        match products {
            Ok(products) => Ok(products),
            Err(e) => Err(ErrorHandlingUtils::application_error("Error getting all products", Some(e))),
        }
    }
}