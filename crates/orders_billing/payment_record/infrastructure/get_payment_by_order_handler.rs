use actix_web::{web, HttpResponse};
use std::sync::Arc;
use serde_json::json;

use crate::payment_record::domain::repository::OrdersBillingRepository;

pub async fn get_payment_by_order_handler(
    repo: web::Data<Arc<dyn OrdersBillingRepository>>,
    path: web::Path<i32>,
) -> HttpResponse {
    let order_id = path.into_inner();

    match repo.get_by_order_id(order_id).await {
        Ok(Some(payment)) => HttpResponse::Ok().json(payment),
        Ok(None) => HttpResponse::NotFound().json(json!({ "message": "No se encontró pago para este pedido" })),
        Err(e) => HttpResponse::InternalServerError().json(json!({ "error": e })),
    }
}
