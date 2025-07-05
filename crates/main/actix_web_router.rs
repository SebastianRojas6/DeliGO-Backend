use actix_web::{HttpResponse, Scope};
use actix_web::web::{self, ServiceConfig};
use crate::auth_user::AuthUser;
use admin::crud_product::infrastructure::api::product_facts::order_facts_controller::*;
/*
use crate::crud_enrollment::infrastructure::controllers::{
    get_schedule_handler,
    withdraw_enrollment_handler,
};

*/

// only need to use the AuthUser struct for now
pub async fn protected_route(user: AuthUser) -> HttpResponse {
    HttpResponse::Ok().body(format!("Hello, {}!", user.0.user_metadata.role))
}

fn products_scope() -> Scope {
    web::scope("/products")
        .route("", web::get().to(get_all_products_facts))
        .route("/{id}", web::get().to(get_product_facts_by_id))
        .route("", web::post().to(create_product_facts))
        .route("", web::patch().to(update_product_facts))
        .route("/{id}", web::delete().to(delete_product_facts))
        .service(web::resource("/protected").route(web::get().to(protected_route)))
}

pub fn configure_enrollment_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/deligo")
            .service(products_scope())
            
        
            
    );
}
