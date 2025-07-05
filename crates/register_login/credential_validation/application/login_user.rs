use crate::credential_validation::domain::{
    repository::UserCredentialRepository,
};
use bcrypt::verify;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Serialize;
use std::sync::Arc;

pub struct LoginUserUseCase {
    pub repo: Arc<dyn UserCredentialRepository>,
    pub jwt_secret: String,
}

impl LoginUserUseCase {
    pub fn new(repo: Arc<dyn UserCredentialRepository>, jwt_secret: String) -> Self {
        Self { repo, jwt_secret }
    }

    pub async fn execute(
        &self,
        address: String,
        password: String,
    ) -> Result<String, String> {
        let user = self
            .repo
            .find_by_address(&address)
            .await?
            .ok_or_else(|| "Usuario no encontrado".to_string())?;

        let is_valid = verify(&password, &user.password)
            .map_err(|e| format!("Error al verificar la contraseña: {}", e))?;

        if !is_valid {
            return Err("Contraseña incorrecta".to_string());
        }

        let claims = JwtClaims {
            sub: user.id.unwrap_or_default().to_string(),
            email: user.address.clone(),
            user_metadata: JwtMetadata {
                role: "user".to_string(), 
            },
            exp: chrono::Utc::now().timestamp() as usize + 60 * 60 * 24, 
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| format!("Error al generar el token: {}", e))?;

        Ok(token)
    }
}

#[derive(Debug, Serialize)]
struct JwtClaims {
    sub: String,
    email: String,
    user_metadata: JwtMetadata,
    exp: usize,
}

#[derive(Debug, Serialize)]
struct JwtMetadata {
    role: String,
}
