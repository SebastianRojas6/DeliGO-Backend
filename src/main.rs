mod db;
mod models;
mod routes;

use crate::db::init_db;
use crate::routes::{orders, products};
use actix_web::{web, App, HttpServer};
use log::{error, info, warn};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    info!("Starting server...");

    let pool = init_db().await.expect("Failed to connect to database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(orders::config)
            .configure(products::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
