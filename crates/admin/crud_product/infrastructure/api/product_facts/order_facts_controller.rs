use actix_web::{get, patch, post, web::Data, web::ServiceConfig, web::Path, HttpResponse, delete};
use shared::config::AppState;
use crate::crud_product::application::dto::product_dto::{ProductCreateDto, ProductUpdateDto};
use crate::crud_product::application::interface::AbstractUseCase;
use crate::crud_product::application::product_usecases::create_product_facts_usecase::CreateProductFactsUseCase;
use crate::crud_product::application::product_usecases::delete_product_facts_usecase::DeleteProductFactsUseCase;
use crate::crud_product::application::product_usecases::get_all_products_facts_usecase::GetAllProductsFactsUseCase;
use crate::crud_product::application::product_usecases::get_product_by_id_facts_usecase::GetProductByIdFactsUsecase;
use crate::crud_product::application::product_usecases::update_product_facts_usecase::UpdateProductFactsUseCase;
use crate::crud_product::infrastructure::api::shared::error_presenter::ErrorResponse;

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(get_all_products_facts);
}

#[get("/")]
async fn get_all_products_facts(data: Data<AppState>) -> Result<HttpResponse, ErrorResponse> {
    let get_all_products_facts_uc = GetAllProductsFactsUseCase::new(&data.crud_product_repo);
    let product_facts = get_all_products_facts_uc.execute().await;
    product_facts
        .map(|facts| HttpResponse::Ok().json(facts))
        .map_err(ErrorResponse::map_io_error)
}

#[get("/{id}")]
async fn get_product_facts_by_id(data: Data<AppState>, id: Path<i32>) -> Result<HttpResponse, ErrorResponse> {
    let get_product_uc = GetProductByIdFactsUsecase::new(&data.crud_product_repo, id.into_inner());
    let product_facts = get_product_uc.execute().await;
    product_facts
        .map(|facts| HttpResponse::Ok().json(facts))
        .map_err(ErrorResponse::map_io_error)
}

#[post("/")]
async fn create_product_facts(data: Data<AppState>, product_create_dto: ProductCreateDto) -> Result<HttpResponse, ErrorResponse> {
    let create_product_uc = CreateProductFactsUseCase::new(&data.crud_product_repo, product_create_dto);
    let product_facts = create_product_uc.execute().await;
    product_facts
        .map(|facts| HttpResponse::Created().json(facts))
        .map_err(ErrorResponse::map_io_error)
}

#[patch("/")]
async fn update_product_facts(data: Data<AppState>, product_update_dto: ProductUpdateDto) -> Result<HttpResponse, ErrorResponse> {
    let update_product_uc = UpdateProductFactsUseCase::new(&data.crud_product_repo, product_update_dto);
    let product_facts = update_product_uc.execute().await;
    product_facts
        .map(|facts| HttpResponse::Ok().json(facts))
        .map_err(ErrorResponse::map_io_error)
}

#[delete("/{id}")]
async fn delete_product_facts(data: Data<AppState>, id: Path<i32>) -> Result<HttpResponse, ErrorResponse> {
    let delete_product_uc = DeleteProductFactsUseCase::new(&data.crud_product_repo, id.into_inner());
    let result = delete_product_uc.execute().await;
    result
        .map(|_| HttpResponse::NoContent().finish())
        .map_err(ErrorResponse::map_io_error)
}

