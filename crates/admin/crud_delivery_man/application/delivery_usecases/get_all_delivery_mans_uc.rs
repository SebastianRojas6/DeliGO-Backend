use std::sync::Arc;
use crate::crud_delivery_man::domain::model::DeliveryManEntity;
use crate::crud_delivery_man::domain::repository::DeliveryManAbstractRepository;
use crate::crud_product::application::interface::AbstractUseCase;

pub struct GetAllDeliveryMansUc {
    repository: Arc<dyn DeliveryManAbstractRepository>,
}

impl GetAllDeliveryMansUc {
    pub fn new(repository: Arc<dyn DeliveryManAbstractRepository>) -> Self {
        GetAllDeliveryMansUc { repository }
    }
}

#[async_trait::async_trait(?Send)]
impl AbstractUseCase<Vec<DeliveryManEntity>> for GetAllDeliveryMansUc {
    async fn execute(&self) -> Result<Vec<DeliveryManEntity>, crate::crud_product::application::interface::ApiError> {
        let delivery_mans = self.repository.get_all_delivery_mans().await;
        match delivery_mans {
            Ok(delivery_mans) => Ok(delivery_mans),
            Err(e) => Err(crate::crud_product::application::utils::ErrorHandlingUtils::application_error("Error getting all delivery mans", Some(e))),
        }
    }
}

