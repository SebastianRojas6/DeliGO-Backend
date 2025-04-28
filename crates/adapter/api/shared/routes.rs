use actix_web::web;
use crate::api::product_facts::product_facts_controllers::routes as product_routes;

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(web::scope("/api/v1").configure(product_routes));
        
}