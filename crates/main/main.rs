use std::env;
use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use adapter::spi::db::db_connection::DbConnectionProvider;
use adapter::api::shared::app_state::AppState;
use adapter::api::shared::routes::routes;
use adapter::spi::db::db_product_facts_repository::ProductFactsRepository;
use adapter::spi::db::db_user_facts_repository::UserFactsRepository;

async fn server(listener: TcpListener) -> Result<Server, std::io::Error> {
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "actix_web=info,actix_server=info");

    env_logger::try_init().expect("TODO: panic message");
    let db_provider = DbConnectionProvider::new().await;
    let data = Data::new(
        AppState {
            app_name: "Skibidi Api".to_string(),
            product_repository: ProductFactsRepository::new(db_provider.get_connection().clone()),
            user_repository: UserFactsRepository::new(db_provider.get_connection().clone())
        }
    );

    let port = listener.local_addr()?.port();

    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            
            .app_data(data.clone())
            .wrap(Logger::default())
            .configure(routes)
    })
        .listen(listener)?
        .run();
    println!("Server running on port {}", port);
    println!("Server running on http://localhost:{}", port);
    
    Ok(server)

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let listener = TcpListener::bind("0.0.0.0:8888").expect("Failed to bind random port");
    let serv = server(listener).await?;
    serv.await

}