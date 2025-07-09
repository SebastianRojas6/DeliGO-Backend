use super::super::config::app_state::AppState;
use crate::domain::repository::UserRepository;
use actix_web::{web, HttpResponse, Responder};

#[derive(serde::Deserialize)]
pub struct RatingRequest {
    pub user_id: i32,
    pub delivery_id: i32,
}

#[derive(serde::Deserialize)]
pub struct ChangeStatusRequest {
    pub id: String,
    pub estado: String,
}

pub async fn rate_delivery_controller(req: web::Json<RatingRequest>, app_state: web::Data<AppState>) -> impl Responder {
    match app_state.user_repo.rating_delivery(req.user_id, req.delivery_id).await {
        Ok(_) => HttpResponse::Ok().json("Rating submitted successfully"),
        Err(e) => HttpResponse::BadRequest().body(e),
    }
}

pub async fn change_order_status_controller(req: web::Json<ChangeStatusRequest>, app_state: web::Data<AppState>) -> impl Responder {
    let result = app_state.user_repo.change_order_status(&req.id, &req.estado).await;

    match result {
        Ok(status_str) => HttpResponse::Ok().json(format!("Estado actualizado: {}", status_str)),
        Err(e) => HttpResponse::BadRequest().body(format!("Error: {}", e)),
    }
}
