use async_trait::async_trait;
use domain::error::ApiError;
use domain::order::OrderEntity;
use crate::repositories::order_fact_repository_abstract::OrderFactRepositoryAbstract;
use crate::usecases::interfaces::AbstractUseCase;
use crate::utils::error_handling_utils::ErrorHandlingUtils;

pub struct GetOneOrderByIdFactUsecase<'a> {
    order_fact_id: &'a i32,
    order_fact_repository: &'a dyn OrderFactRepositoryAbstract,
}

impl<'a> GetOneOrderByIdFactUsecase<'a> {
    pub fn new(order_fact_id: &'a i32, order_fact_repository: &'a dyn OrderFactRepositoryAbstract) -> Self {
        GetOneOrderByIdFactUsecase {
            order_fact_id,
            order_fact_repository,
        }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<OrderEntity> for GetOneOrderByIdFactUsecase<'a> {
    async fn execute(&self) -> Result<OrderEntity, ApiError> {
        let order = self.order_fact_repository.get_one_order_by_id_fact(*self.order_fact_id).await;
        match order {
            Ok(order) => Ok(order),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot get order by id bruh", Some(e))),
        }
    }
}

