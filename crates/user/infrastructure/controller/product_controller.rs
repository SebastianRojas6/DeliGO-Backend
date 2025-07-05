use super::super::config::app_state::AppState;
use crate::domain::repository::ProductRepository;
use actix_web::{web, HttpResponse, Responder};

pub async fn get_all_products(app_state: web::Data<AppState>) -> impl Responder {
    match app_state.product_repo.get_all().await {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

pub async fn get_by_purchase_for_user(app_state: web::Data<AppState>, path: web::Path<(i32, i32)>) -> impl Responder {
    let (user_id, product_id) = path.into_inner();
    match app_state.product_repo.get_by_purchase_for_user(user_id, product_id).await {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

pub async fn get_selected_products(app_state: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let user_id = path.into_inner();
    match app_state.product_repo.get_selected_products(user_id).await {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}
