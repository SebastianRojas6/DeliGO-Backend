mod actix_web_router;
mod auth_user;
mod bootstrap;

use actix_web::{App, HttpServer};
use bootstrap::init_state;
use actix_web_router::configure_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok(); 

    let app_state = init_state().await.expect("Failed to initialize state");

    HttpServer::new(move || {
        App::new()
            .service(configure_routes(&app_state))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
