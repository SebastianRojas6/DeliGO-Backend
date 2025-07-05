use super::super::config::app_state::AppState;
use crate::domain::repository::UserRepository;
use actix_web::{web, HttpResponse, Responder};

#[derive(serde::Deserialize)]
pub struct RatingRequest {
    pub user_id: i32,
    pub delivery_id: i32,
}

pub async fn rate_delivery_controller(req: web::Json<RatingRequest>, user_query: web::Data<AppState>) -> impl Responder {
    match user_query.user_repo.rating_delivery(req.user_id, req.delivery_id).await {
        Ok(_) => HttpResponse::Ok().json("Rating submitted successfully"),
        Err(e) => HttpResponse::BadRequest().body(e),
    }
}
