use std::sync::Arc;
use crate::crud_delivery_man::domain::repository::DeliveryManAbstractRepository;
use crate::crud_product::application::interface::AbstractUseCase;

pub struct AsignDeliveryToOrderUC {
    repository: Arc<dyn DeliveryManAbstractRepository>,
    id_order: i32,
    id_delivery_man: i32,    
}

impl AsignDeliveryToOrderUC {
    pub fn new(
        repository: Arc<dyn DeliveryManAbstractRepository>,
        id_order: i32,
        id_delivery_man: i32)
        -> Self {
        AsignDeliveryToOrderUC {
            repository,
            id_order,
            id_delivery_man
        }
    }
}

#[async_trait::async_trait(?Send)]
impl AbstractUseCase<()> for AsignDeliveryToOrderUC {
    async fn execute(&self) -> Result<(), crate::crud_product::application::interface::ApiError> {
        let result = self.repository.asign_delivery_to_order(self.id_order, self.id_delivery_man).await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(crate::crud_product::application::utils::ErrorHandlingUtils::application_error("Error asigning delivery to order", Some(e))),
        }
    }
}