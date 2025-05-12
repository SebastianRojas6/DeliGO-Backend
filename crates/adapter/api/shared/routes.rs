use actix_web::web;
use crate::api::product_facts::product_facts_controllers::routes as product_routes;
use crate::api::user_facts::user_facts_controllers::routes as user_routes;

const API_VERSION: &str = "v1";
const API_BASE_PATH: &str = "/api";

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope(&format!("{}/{}", API_BASE_PATH, API_VERSION))
            .service(web::scope("/products").configure(product_routes))
            .service(web::scope("/users").configure(user_routes))
    );
}