use std::sync::Arc;
use crate::crud_delivery_man::domain::model::DeliveryManEntity;
use crate::crud_delivery_man::domain::repository::DeliveryManAbstractRepository;
use crate::crud_product::application::interface::AbstractUseCase;

pub struct GetDeliveryInfoByIdUC {
    repository: Arc<dyn DeliveryManAbstractRepository>,
    id: i32,
}

impl GetDeliveryInfoByIdUC {
    pub fn new(repository: Arc<dyn DeliveryManAbstractRepository>, id: i32) -> Self {
        GetDeliveryInfoByIdUC { repository, id }
    }
}

#[async_trait::async_trait(?Send)]
impl AbstractUseCase<DeliveryManEntity> for GetDeliveryInfoByIdUC {
    async fn execute(&self) -> Result<DeliveryManEntity, crate::crud_product::application::interface::ApiError> {
        let delivery_man = self.repository.get_delivery_info_by_id(self.id).await;
        match delivery_man {
            Ok(delivery_man) => Ok(delivery_man),
            Err(e) => Err(crate::crud_product::application::utils::ErrorHandlingUtils::application_error("Error getting delivery by id", Some(e))),
        }
    }
}