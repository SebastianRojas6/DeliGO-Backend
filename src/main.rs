mod models;
mod routes;
mod db;

use actix_web::{web, App, HttpServer};
use crate::db::init_db;
use crate::routes::{orders, products};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

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

