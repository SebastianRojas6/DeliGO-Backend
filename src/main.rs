mod config;
mod db;
mod models;

use models::shared::OrderStatus;
use models::order::OrderRequest;
use db::{mongo, postgres};

fn main() {
    config::init_env();

    // Conexión a PostgreSQL
    let pg_url = config::get_postgres_url();
    let _pg_pool = postgres::connect_postgres(&pg_url).await.expect("Error conectando a PostgreSQL");

    // Conexión a MongoDB
    let mongo_url = config::get_mongo_url();
    let _mongo_client = mongo::connect_mongo(&mongo_url).await.expect("Error conectando a MongoDB");

    println!("Conexiones exitosas");

    let estado = OrderStatus::Preparing;
    
    println!("Estado del pedido: {:?}", estado);

    let solicitud = OrderRequest {
        user_id: 10,
        items: vec![1, 2, 3],
        delivery_address: String::from("Av. Miguelcmep 123"),
    };
    
    
    println!("Usuario ID: {}", solicitud.user_id);
    println!("Items: {:?}", solicitud.items);
    println!("Dirección: {}", solicitud.delivery_address);


}