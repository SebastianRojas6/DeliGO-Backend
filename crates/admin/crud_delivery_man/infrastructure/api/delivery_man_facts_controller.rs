use std::sync::Arc;
use actix_web::{web::Data, web::Json, web::Path, HttpResponse};
use crate::crud_delivery_man::application::delivery_usecases::asign_delivery_to_order_uc::AsignDeliveryToOrderUC;
use crate::crud_delivery_man::application::delivery_usecases::get_all_delivery_mans_uc::GetAllDeliveryMansUc;
use crate::crud_delivery_man::application::delivery_usecases::get_delivery_info_by_id_uc::GetDeliveryInfoByIdUC;
use crate::crud_delivery_man::application::delivery_usecases::update_delivery_man_uc::UpdateDeliveryManUC;
use crate::crud_delivery_man::domain::repository::DeliveryManAbstractRepository;
use crate::crud_delivery_man::application::dto::delivery_man_dto::DeliveryManUpdateDto;
use crate::crud_delivery_man::domain::model::DeliveryManEntity;
use crate::crud_product::application::interface::AbstractUseCase;
use crate::crud_product::infrastructure::api::shared::error_presenter::ErrorResponse;

pub async fn get_all_delivery_men(
    data: Data<Arc<dyn DeliveryManAbstractRepository>>
) -> Result<HttpResponse, ErrorResponse> {
    let getAllDeliveryFactUC = GetAllDeliveryMansUc::new(data.get_ref().clone());
    let result = getAllDeliveryFactUC.execute().await;
    result
        .map(|delivery_men| HttpResponse::Ok().json(delivery_men))
        .map_err(ErrorResponse::map_io_error)
}

pub async fn get_delivery_man_by_id(
    data: Data<Arc<dyn DeliveryManAbstractRepository>>,
    id: Path<i32>
) -> Result<HttpResponse, ErrorResponse> {
    let get_delivery_info_uc = GetDeliveryInfoByIdUC::new(data.get_ref().clone(), id.into_inner());
    let result = get_delivery_info_uc.execute().await;
    result
        .map(|delivery_man| HttpResponse::Ok().json(delivery_man))
        .map_err(ErrorResponse::map_io_error)
}

pub async fn update_delivery_man(
    data: Data<Arc<dyn DeliveryManAbstractRepository>>,
    delivery_man_update_dto: Json<DeliveryManUpdateDto>
) -> Result<HttpResponse, ErrorResponse> {
    let update_delivery_info_uc = UpdateDeliveryManUC::new(data.get_ref().clone(), delivery_man_update_dto.into_inner());
    let result = update_delivery_info_uc.execute().await;
    result
        .map(|delivery_man| HttpResponse::Ok().json(delivery_man))
        .map_err(ErrorResponse::map_io_error)
}

pub async fn assign_delivery_to_order(
    data: Data<Arc<dyn DeliveryManAbstractRepository>>,
    id_delivery: Path<(i32, i32)>
) -> Result<HttpResponse, ErrorResponse> {
    let (id_delivery, id_order) = id_delivery.into_inner();
    let asign_delivery_uc = AsignDeliveryToOrderUC::new(data.get_ref().clone(), id_order, id_delivery);
    let result = asign_delivery_uc.execute().await;
    result
        .map(|_| HttpResponse::NoContent().finish())
        .map_err(ErrorResponse::map_io_error)
}