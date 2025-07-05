use crate::credential_validation::domain::{
    model::UserCredential,
    repository::UserCredentialRepository,
};
use std::sync::Arc;
use bcrypt::{hash, DEFAULT_COST};

pub struct RegisterUserUseCase {
    pub repo: Arc<dyn UserCredentialRepository>,
}

impl RegisterUserUseCase {
    pub fn new(repo: Arc<dyn UserCredentialRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        name: String,
        phone: String,
        email: String,
        address: String, 
        raw_password: String,
    ) -> Result<(), String> {
        if let Some(_) = self.repo.find_by_email(&email).await? {
            return Err("El usuario ya está registrado".to_string());
        }

        let hashed_password = hash(raw_password, DEFAULT_COST)
            .map_err(|e| format!("Error al hashear la contraseña: {}", e))?;

        let user = UserCredential {
            id: None,
            name,
            phone,
            email,
            address, 
            password: hashed_password,
        };

        self.repo.create_user(user).await
    }
}
