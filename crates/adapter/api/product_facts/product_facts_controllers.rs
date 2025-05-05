use actix_web::{web, get, HttpResponse};
use actix_web::web::ServiceConfig;
use application::mappers::api_mapper::ApiMapper;
use application::usecases::get_all_product_facts_usecase::GetAllProductUSeCase;
use application::usecases::get_one_product_fact_by_id_usecase::GetOneProductByIdUseCase;
use application::usecases::interfaces::AbstractUseCase;
use crate::api::product_facts::product_facts_mappers::ProductFactPresenterMapper;
use crate::api::product_facts::product_facts_presenters::ProductFactPresenter;
use crate::api::shared::app_state::AppState;
use crate::api::shared::error_presenter::ErrorResponse;

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/product_facts")
        .service(get_all_products_facts)
        .service(get_one_product_fact_by_id));
}

#[get("/")]
async fn get_all_products_facts(data: web::Data<AppState>) -> Result<HttpResponse, ErrorResponse> {
    let get_all_product_facts_usecase = GetAllProductUSeCase::new(&data.product_repository);
    let product_facts = get_all_product_facts_usecase.execute().await;
    product_facts.map_err(ErrorResponse::map_io_error)
        .map(|facts| {
            HttpResponse::Ok().json(facts.into_iter().map(ProductFactPresenterMapper::to_api).collect::<Vec<ProductFactPresenter>>())
        })
}

#[get("/{fact_id}")]
async fn get_one_product_fact_by_id(data : web::Data<AppState>, fact_id: web::Path<i32>) -> Result<HttpResponse, ErrorResponse> {
    let get_one_product_by_id_usecase = GetOneProductByIdUseCase::new(&fact_id, &data.product_repository);
    let product_fact = get_one_product_by_id_usecase.execute().await;
    product_fact.map_err(ErrorResponse::map_io_error)
        .map(|fact| {
            HttpResponse::Ok().json(ProductFactPresenterMapper::to_api(fact))
        })
}

