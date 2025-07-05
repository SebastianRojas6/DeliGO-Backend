use actix_web::{HttpResponse, web};
use std::sync::Arc;
use serde_json::json;

use crate::payment_record::application::generate_invoice::GenerateInvoiceUseCase;
use crate::payment_record::domain::repository::InvoiceGenerator;

pub async fn generate_invoice_handler(
    repo: web::Data<Arc<dyn InvoiceGenerator>>,
    path: web::Path<i32>,
) -> HttpResponse {
    let order_id = path.into_inner();
    let use_case = GenerateInvoiceUseCase::new(repo.get_ref().clone());

    match use_case.execute(order_id).await {
        Ok(invoice) => HttpResponse::Ok().json(invoice),
        Err(e) => HttpResponse::BadRequest().json(json!({ "error": e })),
    }
}
