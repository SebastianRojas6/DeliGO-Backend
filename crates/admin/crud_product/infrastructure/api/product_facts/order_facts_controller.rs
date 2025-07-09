use crate::crud_product::application::dto::product_dto::{ProductCreateDto, ProductUpdateDto};
use crate::crud_product::application::interface::AbstractUseCase;
use crate::crud_product::application::product_usecases::create_product_facts_usecase::CreateProductFactsUseCase;
use crate::crud_product::application::product_usecases::delete_product_facts_usecase::DeleteProductFactsUseCase;
use crate::crud_product::application::product_usecases::get_all_products_facts_usecase::GetAllProductsFactsUseCase;
use crate::crud_product::application::product_usecases::get_product_by_id_facts_usecase::GetProductByIdFactsUsecase;
use crate::crud_product::application::product_usecases::update_product_facts_usecase::UpdateProductFactsUseCase;
use crate::crud_product::domain::repository::ProductAbstractRepository;
use crate::crud_product::infrastructure::api::shared::error_presenter::ErrorResponse;
use actix_web::web::Json;
use actix_web::{web::Data, web::Path, HttpResponse};
use std::sync::Arc;

pub async fn get_all_products_facts(data: Data<Arc<dyn ProductAbstractRepository>>) -> Result<HttpResponse, ErrorResponse> {
    let get_all_products_facts_uc = GetAllProductsFactsUseCase::new(data.get_ref().clone());
    let product_facts = get_all_products_facts_uc.execute().await;
    product_facts.map(|facts| HttpResponse::Ok().json(facts)).map_err(ErrorResponse::map_io_error)
}

pub async fn get_product_facts_by_id(data: Data<Arc<dyn ProductAbstractRepository>>, id: Path<i32>) -> Result<HttpResponse, ErrorResponse> {
    let get_product_uc = GetProductByIdFactsUsecase::new(data.get_ref().clone(), id.into_inner());
    let product_facts = get_product_uc.execute().await;
    product_facts.map(|facts| HttpResponse::Ok().json(facts)).map_err(ErrorResponse::map_io_error)
}

pub async fn create_product_facts(data: Data<Arc<dyn ProductAbstractRepository>>, product_create_dto: Json<ProductCreateDto>) -> Result<HttpResponse, ErrorResponse> {
    let create_product_uc = CreateProductFactsUseCase::new(data.get_ref().clone(), product_create_dto.into_inner());
    let product_facts = create_product_uc.execute().await;
    product_facts.map(|facts| HttpResponse::Created().json(facts)).map_err(ErrorResponse::map_io_error)
}

pub async fn update_product_facts(data: Data<Arc<dyn ProductAbstractRepository>>, product_update_dto: Json<ProductUpdateDto>) -> Result<HttpResponse, ErrorResponse> {
    let update_product_uc = UpdateProductFactsUseCase::new(data.get_ref().clone(), product_update_dto.into_inner());
    let product_facts = update_product_uc.execute().await;
    product_facts.map(|facts| HttpResponse::Ok().json(facts)).map_err(ErrorResponse::map_io_error)
}

pub async fn delete_product_facts(data: Data<Arc<dyn ProductAbstractRepository>>, id: Path<i32>) -> Result<HttpResponse, ErrorResponse> {
    let delete_product_uc = DeleteProductFactsUseCase::new(data.get_ref().clone(), id.into_inner());
    let result = delete_product_uc.execute().await;
    result.map(|_| HttpResponse::NoContent().finish()).map_err(ErrorResponse::map_io_error)
}
