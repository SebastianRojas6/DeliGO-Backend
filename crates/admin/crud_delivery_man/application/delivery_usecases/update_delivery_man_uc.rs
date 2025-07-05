use std::sync::Arc;
use crate::crud_delivery_man::application::dto::delivery_man_dto::DeliveryManUpdateDto;
use crate::crud_delivery_man::domain::model::DeliveryManEntity;
use crate::crud_delivery_man::domain::repository::DeliveryManAbstractRepository;
use crate::crud_product::application::interface::AbstractUseCase;

pub struct UpdateDeliveryManUC {
    repository: Arc<dyn DeliveryManAbstractRepository>,
    delivery_man_update_dto: DeliveryManUpdateDto
}

impl UpdateDeliveryManUC {
    pub fn new(
        repository: Arc<dyn DeliveryManAbstractRepository>,
        delivery_man_update_dto: DeliveryManUpdateDto) -> Self {
        UpdateDeliveryManUC {
            repository,
            delivery_man_update_dto,
        }
    }
}

#[async_trait::async_trait(?Send)]
impl AbstractUseCase<DeliveryManEntity> for UpdateDeliveryManUC {
    async fn execute(&self) -> Result<DeliveryManEntity, crate::crud_product::application::interface::ApiError> {
        let delivery_man = self.repository.update_delivery_man(self.delivery_man_update_dto.clone()).await;

        match delivery_man {
            Ok(deliver_man) => Ok(deliver_man),
            Err(e) => {
                Err(crate::crud_product::application::utils::ErrorHandlingUtils::application_error(
                    "Error updating delivery_man info", Some(e)))
            }
        }
    }
}