use async_trait::async_trait;
use domain::error::ApiError;
use domain::order::{OrderEntity};
use crate::repositories::order_fact_repository_abstract::OrderFactRepositoryAbstract;
use crate::usecases::interfaces::AbstractUseCase;
use crate::utils::error_handling_utils::ErrorHandlingUtils;

pub struct GetAllOrderUsecase<'a> {
    repository: &'a dyn OrderFactRepositoryAbstract,
}

impl <'a> GetAllOrderUsecase<'a> {
    pub fn new(repository: &'a dyn OrderFactRepositoryAbstract) -> Self{
        GetAllOrderUsecase{ repository }
    }
}

#[async_trait(?Send)]
impl <'a>AbstractUseCase<Vec<OrderEntity>> for GetAllOrderUsecase<'a> {
    async fn execute(&self) -> Result<Vec<OrderEntity>, ApiError> {
        let orders = self.repository.get_all_order_facts().await;
        match orders { 
            Ok(orders) => Ok(orders),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot get all orders bruh", Some(e)))
        }
    }
    
}