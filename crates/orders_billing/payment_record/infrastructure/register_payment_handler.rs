use actix_web::{HttpResponse, web};
use serde::Deserialize;
use std::sync::Arc;
use serde_json::json;

use crate::payment_record::application::register_payment::RegisterPaymentUseCase;
use crate::payment_record::domain::model::PaymentMethod;
use crate::payment_record::domain::repository::OrdersBillingRepository;

#[derive(Debug, Deserialize)]
pub struct RegisterPaymentInput {
    pub order_id: i32,
    pub total_amount: f64,
    pub payment_method: PaymentMethod,
}

pub async fn register_payment_handler(
    repo: web::Data<Arc<dyn OrdersBillingRepository>>,
    input: web::Json<RegisterPaymentInput>,
) -> HttpResponse {
    let use_case = RegisterPaymentUseCase::new(repo.get_ref().clone());

    match use_case
        .execute(input.order_id, input.total_amount, input.payment_method.clone())
        .await
    {
        Ok(_) => HttpResponse::Ok().json(json!({ "message": "Pago registrado correctamente" })),
        Err(e) => HttpResponse::BadRequest().json(json!({ "error": e })),
    }
}
