use crate::repositories::product_fact_repository_abstract::ProductFactRepositoryAbstract;
use crate::usecases::interfaces::AbstractUseCase;

pub struct DeleteProductByIdFactUseCase<'a> {
    product_fact_id: &'a i32,
    repository: &'a dyn ProductFactRepositoryAbstract,
}

impl<'a> DeleteProductByIdFactUseCase<'a> {
    pub fn new(product_fact_id: &'a i32, repository: &'a dyn ProductFactRepositoryAbstract) -> Self {
        DeleteProductByIdFactUseCase { product_fact_id, repository }
    }
}

#[async_trait::async_trait(?Send)]
impl<'a> AbstractUseCase<()> for DeleteProductByIdFactUseCase<'a> {
    async fn execute(&self) -> Result<(), domain::error::ApiError> {
        let result = self.repository.delete_product_by_id_fact(*self.product_fact_id).await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(crate::utils::error_handling_utils::ErrorHandlingUtils::application_error("Cannot delete product fact", Some(e))),
        }
    }
    
}