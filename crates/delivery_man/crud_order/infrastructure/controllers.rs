use crate::crud_order::application::dtos::{OrderCreateDto, OrderUpdateDto};
use crate::crud_order::application::ucs::OrderUseCase;
use crate::crud_order::domain::repository::OrderAbstractRepository;
use actix_web::web::Path;
use actix_web::{web, HttpResponse};
use std::sync::Arc;

pub async fn list_orders(repo: web::Data<Arc<dyn OrderAbstractRepository>>) -> HttpResponse {
    let get = OrderUseCase::new(repo.get_ref().clone());
    match get.list().await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "error": e.to_string() })),
    }
}

pub async fn create_order(repo: web::Data<Arc<dyn OrderAbstractRepository>>, dto: web::Json<OrderCreateDto>) -> HttpResponse {
    let use_case = OrderUseCase::new(repo.get_ref().clone());
    match use_case.create(dto.into_inner()).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "error": e.to_string() })),
    }
}

pub async fn update_order(repo: web::Data<Arc<dyn OrderAbstractRepository>>, dto: web::Json<OrderUpdateDto>) -> HttpResponse {
    let use_case = OrderUseCase::new(repo.get_ref().clone());
    match use_case.update(dto.into_inner()).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "error": e.to_string() })),
    }
}

pub async fn delete_order(repo: web::Data<Arc<dyn OrderAbstractRepository>>, id: Path<i32>) -> HttpResponse {
    let use_case = OrderUseCase::new(repo.get_ref().clone());
    match use_case.delete(id.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({ "message": "Order deleted successfully" })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "error": e.to_string() })),
    }
}
