use actix_web::{web, HttpResponse, Scope};
use delivery_man::crud_order::infrastructure::controllers::*;
use std::sync::Arc;

use crate::{auth_user::AuthUser, bootstrap::AppState};

// Registro-Login
use register_login::credential_validation::infrastructure::{login_handler::login_handler, register_handler::register_handler};

// Orders-Billing
use orders_billing::payment_record::{
    domain::repository::OrdersBillingRepository,
    infrastructure::{generate_invoice_handler::generate_invoice_handler, get_payment_by_order_handler::get_payment_by_order_handler, register_payment_handler::register_payment_handler},
};

// User
use user::infrastructure::controller::{
    product_controller::{get_by_purchase_for_user, get_selected_products},
    user_controller::rate_delivery_controller,
};

// Admin
use admin::crud_product::infrastructure::api::product_facts::order_facts_controller::{
    create_product_facts, delete_product_facts, get_all_products_facts, get_product_facts_by_id, update_product_facts,
};

pub async fn protected_route(user: AuthUser) -> HttpResponse {
    HttpResponse::Ok().body(format!("Hello, {}!", user.0.user_metadata.role))
}

pub fn configure_routes(app_state: &AppState) -> Scope {
    web::scope("/deligo")
        // Estado compartido
        .app_data(web::Data::new(app_state.clone()))
        .app_data(web::Data::new(Arc::clone(&app_state.credential_repo)))
        .app_data(web::Data::new(Arc::clone(&app_state.products_repo)))
        .app_data(web::Data::new(Arc::clone(&app_state.delivery_man_repo)))
        .app_data(web::Data::new(app_state.orders_billing_repo.clone() as Arc<dyn OrdersBillingRepository>))
        .app_data(web::Data::new(app_state.jwt_secret.clone()))
        .app_data(web::Data::new(app_state.order_repo.clone()))
        // Rutas protegidas
        .route("/protected", web::get().to(protected_route))
        // Registro-Login
        .route("/register", web::post().to(register_handler))
        .route("/login", web::post().to(login_handler))
        // Orders-Billing
        .route("/payments", web::post().to(register_payment_handler))
        .route("/invoice/{order_id}", web::get().to(generate_invoice_handler))
        .route("/payments/{order_id}", web::get().to(get_payment_by_order_handler))
        // Usuario (cliente)
        .route("/rate-delivery", web::post().to(rate_delivery_controller))
        .route("/products", web::get().to(get_all_products_facts))
        .route("/products/purchase/{user_id}/{product_id}", web::get().to(get_by_purchase_for_user))
        .route("/products/selected/{user_id}", web::get().to(get_selected_products))
        // Admin - Product Facts
        .route("/products/{id}", web::get().to(get_product_facts_by_id))
        .route("/products", web::post().to(create_product_facts))
        .route("/products", web::patch().to(update_product_facts))
        .route("/products/{id}", web::delete().to(delete_product_facts))
        // order routes
        .route("/orders", web::get().to(list_orders))
        .route("/orders", web::post().to(create_order))
        .route("/orders", web::patch().to(update_order))
        .route("/orders/{id}", web::delete().to(delete_order))
}
