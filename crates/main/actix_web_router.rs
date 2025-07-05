use actix_web::{HttpResponse, Scope};
use actix_web::web::{self, ServiceConfig};
use crate::auth_user::AuthUser;
use crate::bootstrap::AppState;
use actix_web::web;
use std::sync::Arc;
use admin::crud_product::infrastructure::api::product_facts::order_facts_controller::*;
use register_login::credential_validation::infrastructure::login_handler::login_handler;
use register_login::credential_validation::infrastructure::register_handler::register_handler;
use orders_billing::payment_record::infrastructure::generate_invoice_handler::generate_invoice_handler;
use orders_billing::payment_record::infrastructure::get_payment_by_order_handler::get_payment_by_order_handler;
use orders_billing::payment_record::infrastructure::register_payment_handler::register_payment_handler;

use user::infrastructure::controller::{
    product_controller::{get_all_products, get_by_purchase_for_user, get_selected_products},
    user_controller::rate_delivery_controller,
};

pub async fn protected_route(user: AuthUser) -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().body(format!("Hello, {}!", user.0.user_metadata.role))
}

pub fn configure_routes(app_state: &AppState) -> actix_web::Scope {
    web::scope("/deligo")
        .app_data(web::Data::new(app_state.clone()))
        .app_data(web::Data::new(Arc::clone(&app_state.credential_repo)))
        .app_data(web::Data::new(Arc::clone(&app_state.products_repo)))
        .app_data(web::Data::new(app_state.jwt_secret.clone()))
        .route("/protected", web::get().to(protected_route))
        .route("/register", web::post().to(register_handler))
        .route("/login", web::post().to(login_handler))
        .route("/payments", web::post().to(register_payment_handler))
        .route("/invoice/{order_id}", web::get().to(generate_invoice_handler))
        .route("/payments/{order_id}", web::get().to(get_payment_by_order_handler))
        .route("/rate-delivery", web::post().to(rate_delivery_controller))
        .route("/products", web::get().to(get_all_products))
        .route("/products/purchase/{user_id}/{product_id}", web::get().to(get_by_purchase_for_user))
        .route("/products/selected/{user_id}", web::get().to(get_selected_products))
        .route("/products/{id}", web::get().to(get_product_facts_by_id))
        .route("/products", web::post().to(create_product_facts))
        .route("/products", web::patch().to(update_product_facts))
        .route("/products/{id}", web::delete().to(delete_product_facts))
}
