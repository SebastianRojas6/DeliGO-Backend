use actix_web::web;
use std::sync::Arc;

use crate::bootstrap::AppState;
use crate::auth_user::AuthUser;
use register_login::credential_validation::infrastructure::login_handler::login_handler;
use register_login::credential_validation::infrastructure::register_handler::register_handler;

pub async fn protected_route(user: AuthUser) -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().body(format!("Hello, {}!", user.0.user_metadata.role))
}

pub fn configure_routes(app_state: &AppState) -> actix_web::Scope {
    web::scope("/deligo")
        .app_data(web::Data::new(app_state.clone()))
        .app_data(web::Data::new(Arc::clone(&app_state.credential_repo)))
        .app_data(web::Data::new(app_state.jwt_secret.clone()))

        .route("/protected", web::get().to(protected_route))
        
        .route("/register", web::post().to(register_handler))
        .route("/login", web::post().to(login_handler))
}