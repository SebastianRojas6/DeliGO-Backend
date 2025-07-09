mod actix_web_router;
mod auth_user;
mod bootstrap;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use actix_web_router::configure_routes;
use bootstrap::init_state;

// Importa el bootstrap del crate user
use user::infrastructure::config::boostrap::bootstrap_user;
use user::infrastructure::routes::config;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let app_state = init_state().await.expect("Failed to initialize state");
    let user_app_state = bootstrap_user().await.expect("Failed to initialize user state");

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:5173")
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec!["Content-Type", "Authorization"])
                    .max_age(3600),
            )
            .service(configure_routes(&app_state))
            .service(web::scope("/user").app_data(web::Data::new(user_app_state.clone())).configure(config))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
