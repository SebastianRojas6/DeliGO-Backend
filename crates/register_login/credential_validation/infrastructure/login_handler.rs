use actix_web::{HttpResponse, web};
use serde::Deserialize;
use std::sync::Arc;
use crate::credential_validation::application::login_user::LoginUserUseCase;
use crate::credential_validation::domain::repository::UserCredentialRepository;

#[derive(Debug, Deserialize)]
pub struct LoginInput {
    pub address: String,
    pub password: String,
}

pub async fn login_handler(
    repo: web::Data<Arc<dyn UserCredentialRepository>>,
    jwt_secret: web::Data<String>,
    input: web::Json<LoginInput>,
) -> HttpResponse {
    let use_case = LoginUserUseCase::new(repo.get_ref().clone(), jwt_secret.get_ref().clone());

    match use_case
        .execute(input.address.clone(), input.password.clone())
        .await
    {
        Ok(token) => HttpResponse::Ok().json(serde_json::json!({ "token": token })),
        Err(err) => HttpResponse::BadRequest().json(serde_json::json!({ "error": err })),
    }
}
