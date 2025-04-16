use std::env;
use dotenvy::dotenv;

pub fn init_env() {
    dotenv().ok();
}

pub fn get_postgres_url() -> String {
    env::var("POSTGRES_URL").expect("POSTGRES_URL no encontrada")
}

pub fn get_mongo_url() -> String {
    env::var("MONGO_URL").expect("MONGO_URL no encontrada")
}