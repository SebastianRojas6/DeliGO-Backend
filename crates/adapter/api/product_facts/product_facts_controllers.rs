use crate::api::product_facts::product_facts_mappers::ProductFactPresenterMapper;
use crate::api::product_facts::product_facts_payloads::InsertProductFactPayload;
use crate::api::product_facts::product_facts_presenters::ProductFactPresenter;
use crate::api::shared::app_state::AppState;
use crate::api::shared::error_presenter::ErrorResponse;
use actix_web::web::ServiceConfig;
use actix_web::{delete, get, post, web, HttpResponse};
use application::mappers::api_mapper::ApiMapper;
use application::usecases::interfaces::AbstractUseCase;
use application::usecases::product_fact_uc::create_product_fact_usecase::CreateProductFactUseCase;
use application::usecases::product_fact_uc::delete_product_by_id_fact_usecase::DeleteProductByIdFactUseCase;
use application::usecases::product_fact_uc::get_all_product_facts_usecase::GetAllProductUSeCase;
use application::usecases::product_fact_uc::get_one_product_fact_by_id_usecase::GetOneProductByIdUseCase;

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(get_all_products_facts).service(get_one_product_fact_by_id).service(delete_product_fact_by_id).service(create_product_fact);
}

#[get("/")]
async fn get_all_products_facts(data: web::Data<AppState>) -> Result<HttpResponse, ErrorResponse> {
    let get_all_product_facts_usecase = GetAllProductUSeCase::new(&data.product_repository);
    let product_facts = get_all_product_facts_usecase.execute().await;
    product_facts
        .map_err(ErrorResponse::map_io_error)
        .map(|facts| HttpResponse::Ok().json(facts.into_iter().map(ProductFactPresenterMapper::to_api).collect::<Vec<ProductFactPresenter>>()))
}

#[post("/")]
async fn create_product_fact(data: web::Data<AppState>, payload: web::Json<InsertProductFactPayload>) -> Result<HttpResponse, ErrorResponse> {
    let product_fact = ProductFactPresenterMapper::to_dto(payload.into_inner());
    let create_product_fact_usecase = CreateProductFactUseCase::new(&product_fact, &data.product_repository);
    let result = create_product_fact_usecase.execute().await;
    result
        .map_err(ErrorResponse::map_io_error)
        .map(|fact| HttpResponse::Created().json(ProductFactPresenterMapper::to_api(fact)))
}

#[get("/{fact_id}")]
async fn get_one_product_fact_by_id(data: web::Data<AppState>, fact_id: web::Path<i32>) -> Result<HttpResponse, ErrorResponse> {
    let get_one_product_by_id_usecase = GetOneProductByIdUseCase::new(&fact_id, &data.product_repository);
    let product_fact = get_one_product_by_id_usecase.execute().await;
    product_fact
        .map_err(ErrorResponse::map_io_error)
        .map(|fact| HttpResponse::Ok().json(ProductFactPresenterMapper::to_api(fact)))
}

#[delete("/{fact_id}")]
async fn delete_product_fact_by_id(data: web::Data<AppState>, fact_id: web::Path<i32>) -> Result<HttpResponse, ErrorResponse> {
    let delete_product_fact_usecase = DeleteProductByIdFactUseCase::new(&fact_id, &data.product_repository);
    let result = delete_product_fact_usecase.execute().await;
    result.map_err(ErrorResponse::map_io_error).map(|_| HttpResponse::NoContent().finish())
}
