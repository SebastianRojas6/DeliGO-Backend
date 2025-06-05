use crate::api::shared::app_state::AppState;
use crate::api::shared::error_presenter::ErrorResponse;
use crate::api::user_facts::user_facts_mappers::{UserFactsMapper};
use crate::api::user_facts::user_facts_payloads::{CreateUserFactPayload, UpdateUserFactPayload};
use crate::api::user_facts::user_facts_presenters::UserFactsPresenter;
use actix_web::web::ServiceConfig;
use actix_web::{delete, get, patch, post, web, HttpResponse};
use application::mappers::api_mapper::{ApiInMapper, ApiOutMapper};
use application::usecases::interfaces::AbstractUseCase;
use application::usecases::user_fact_uc::create_user_fact_uc::CreateUserFactUseCase;
use application::usecases::user_fact_uc::delete_user_fact_by_id_uc::DeleteUserFactByIdUseCase;
use application::usecases::user_fact_uc::get_all_user_facts_uc::GetAllUserFactsUseCase;
use application::usecases::user_fact_uc::get_one_user_by_id_fact_uc::GetOneUserByIdFactUseCase;
use application::usecases::user_fact_uc::update_user_fact_uc::UpdateUserFactUseCase;

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(get_all_user_facts)
        .service(get_one_user_fact_by_id)
        .service(create_user_fact)
        .service(update_user_fact)
        .service(delete_user_fact_by_id);
}

#[get("/")]
async fn get_all_user_facts(data: web::Data<AppState>) -> Result<HttpResponse, ErrorResponse> {
    let get_all_user_facts_usecase = GetAllUserFactsUseCase::new(&data.user_repository);
    let user_facts = get_all_user_facts_usecase.execute().await;
    user_facts
        .map_err(ErrorResponse::map_io_error)
        .map(|facts| HttpResponse::Ok().json(facts.into_iter().map(UserFactsMapper::to_presenter).collect::<Vec<UserFactsPresenter>>()))
}

#[get("/{fact_id}")]
async fn get_one_user_fact_by_id(data: web::Data<AppState>, fact_id: web::Path<i32>) -> Result<HttpResponse, ErrorResponse> {
    let get_one_user_by_id_usecase = GetOneUserByIdFactUseCase::new(&fact_id, &data.user_repository);
    let user_fact = get_one_user_by_id_usecase.execute().await;
    user_fact
        .map_err(ErrorResponse::map_io_error)
        .map(|fact| HttpResponse::Ok().json(UserFactsMapper::to_presenter(fact)))
}

#[post("/")]
async fn create_user_fact(data: web::Data<AppState>, payload: web::Json<CreateUserFactPayload>) -> Result<HttpResponse, ErrorResponse> {
    let user_fact = UserFactsMapper::to_api(payload.into_inner());
    let create_user_fact_usecase = CreateUserFactUseCase::new(&user_fact, &data.user_repository);
    let result = create_user_fact_usecase.execute().await;
    result
        .map_err(ErrorResponse::map_io_error)
        .map(|fact| HttpResponse::Created().json(UserFactsMapper::to_presenter(fact)))
}

#[patch("/{fact_id}")]
async fn update_user_fact(data: web::Data<AppState>, fact_id: web::Path<i32>, payload: web::Json<UpdateUserFactPayload>) -> Result<HttpResponse, ErrorResponse> {
    let user_fact = UserFactsMapper::to_api(payload.into_inner());
    let update_user_fact_usecase = UpdateUserFactUseCase::new(*fact_id, &user_fact, &data.user_repository);
    let result = update_user_fact_usecase.execute().await;
    result
        .map_err(ErrorResponse::map_io_error)
        .map(|fact| HttpResponse::Ok().json(UserFactsMapper::to_presenter(fact)))
}

#[delete("/{fact_id}")]
async fn delete_user_fact_by_id(data: web::Data<AppState>, fact_id: web::Path<i32>) -> Result<HttpResponse, ErrorResponse> {
    let delete_user_fact_usecase = DeleteUserFactByIdUseCase::new(&fact_id, &data.user_repository);
    let result = delete_user_fact_usecase.execute().await;
    result.map_err(ErrorResponse::map_io_error).map(|_| HttpResponse::NoContent().finish())
}
