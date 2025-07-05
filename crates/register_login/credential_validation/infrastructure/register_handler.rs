use actix_web::{HttpResponse, web};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

use crate::credential_validation::application::register_user::RegisterUserUseCase;
use crate::credential_validation::domain::repository::UserCredentialRepository;

#[derive(Debug, Deserialize)]
pub struct RegisterInput {
    pub name: String,
    pub phone: String,
    pub address: String,
    pub password: String,
}

pub async fn register_handler(
    repo: web::Data<Arc<dyn UserCredentialRepository>>,
    input: web::Json<RegisterInput>,
) -> HttpResponse {
    let use_case = RegisterUserUseCase::new(repo.get_ref().clone());

    match use_case
        .execute(
            input.name.clone(),
            input.phone.clone(),
            input.address.clone(),
            input.password.clone(),
        )
        .await
    {
        Ok(_) => HttpResponse::Ok().json(json!({ "message": "Usuario registrado correctamente" })),
        Err(err) => HttpResponse::BadRequest().json(json!({ "error": err.to_string() })),
    }
}
