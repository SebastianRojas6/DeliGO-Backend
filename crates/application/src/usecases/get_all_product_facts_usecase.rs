use async_trait::async_trait;
use domain::error::ApiError;
use crate::repositories::product_fact_repository_abstract::ProductFactRepositoryAbstract;
use crate::usecases::interfaces::AbstractUseCase;
use domain::product::{Model as ProductModel, ProductResponse};
use crate::utils::error_handling_utils::ErrorHandlingUtils;

pub struct GetAllProductUSeCase<'a> {
    repository: &'a dyn ProductFactRepositoryAbstract,
}

impl <'a> GetAllProductUSeCase<'a> {
    pub fn new(repository: &'a dyn ProductFactRepositoryAbstract) -> Self{ 
        GetAllProductUSeCase { repository }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<Vec<ProductResponse>> for GetAllProductUSeCase<'a> {
    async fn execute(&self) -> Result<Vec<ProductResponse>, ApiError> {
        let fact_products = self.repository.get_all_product_facts().await;
        match fact_products { 
            Ok(facts) => Ok(facts),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot get all cat facts", Some(e))),
        }
    }
}