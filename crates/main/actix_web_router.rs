use actix_web::HttpResponse;
use actix_web::web::{self, ServiceConfig};
use crate::auth_user::AuthUser;
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

pub fn configure_enrollment_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/deligo")
            .route(
                "/protected",
                web::get().to(protected_route)
            )
        
            
    );
}
