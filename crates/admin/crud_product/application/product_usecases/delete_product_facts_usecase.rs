use crate::crud_product::application::interface::{AbstractUseCase, ApiError};
use crate::crud_product::application::utils::ErrorHandlingUtils;
use crate::crud_product::domain::repository::ProductAbstractRepository;

pub struct DeleteProductFactsUseCase<'a> {
    pub product_repository: &'a dyn ProductAbstractRepository,
    pub id_product: i32,
}

impl<'a> DeleteProductFactsUseCase<'a> {
    pub fn new(product_repository: &'a dyn ProductAbstractRepository, id_product: i32) -> Self {
        DeleteProductFactsUseCase {
            product_repository,
            id_product,
        }
    }
}

#[async_trait::async_trait(?Send)]
impl<'a> AbstractUseCase<()> for DeleteProductFactsUseCase<'a> {
    async fn execute(&self) -> Result<(), ApiError> {
        let result = self.product_repository.delete_product(self.id_product).await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(ErrorHandlingUtils::application_error("Error deleting product facts", Some(e))),
        }
    }
}