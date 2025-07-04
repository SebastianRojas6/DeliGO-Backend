mod bootstrap;
mod actix_web_router;
mod auth_user;

use actix_web::{App, HttpServer};
use bootstrap::init_state;
use actix_web_router::configure_enrollment_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok(); 

    let app_state = init_state().await.expect("Failed to initialize state");

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(app_state.clone()))
            .configure(configure_enrollment_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
