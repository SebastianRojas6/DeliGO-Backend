use crate::crud_order::application::dtos::{OrderCreateDto, OrderUpdateDto};
use crate::crud_order::application::ucs::OrderUseCase;
use actix_web::{web, HttpResponse};
use std::sync::Arc;

pub async fn list_orders(use_case: web::Data<Arc<OrderUseCase>>) -> HttpResponse {
    match use_case.list().await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "error": e.to_string() })),
    }
}

pub async fn create_order(use_case: web::Data<Arc<OrderUseCase>>, dto: web::Json<OrderCreateDto>) -> HttpResponse {
    match use_case.create(dto.into_inner()).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "error": e.to_string() })),
    }
}

pub async fn update_order(use_case: web::Data<Arc<OrderUseCase>>, dto: web::Json<OrderUpdateDto>) -> HttpResponse {
    match use_case.update(dto.into_inner()).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "error": e.to_string() })),
    }
}

pub async fn delete_order(use_case: web::Data<Arc<OrderUseCase>>, id: i32) -> HttpResponse {
    match use_case.delete(id).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({ "message": "Order deleted successfully" })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "error": e.to_string() })),
    }
}
