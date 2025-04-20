mod db;
mod models;
mod routes;

use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use crate::routes::orders::OrderController;
use crate::routes::products::ProductController;
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    info!("Starting server...");

    let database_url = "-";
    let pool = PgPool::connect(&database_url).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/orders", web::post().to(OrderController::create_order))
            .route("/orders", web::get().to(OrderController::get_all_orders))
            .route("/orders/{id}", web::get().to(OrderController::get_order_by_id))
            .route("/orders/{id}/status", web::put().to(OrderController::update_order_status))
            .route("/orders/{id}", web::delete().to(OrderController::delete_order))
            .route("/products", web::post().to(ProductController::create_product))
            .route("/products", web::get().to(ProductController::get_all_products))
            .route("/products/{id}", web::get().to(ProductController::get_product_by_id))
            .route("/products/{id}", web::put().to(ProductController::update_product))
            .route("/products/{id}", web::delete().to(ProductController::delete_product))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
