use actix_web::{
    dev::Payload,
    error::ErrorUnauthorized,
    web, Error, FromRequest, HttpRequest,
};
use futures_util::future::BoxFuture;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::Deserialize;
use futures_util::FutureExt;
use crate::bootstrap::AppState;

#[derive(Debug, Deserialize)]
pub struct MetadataSchema {
    pub email: String,
    pub email_verified: bool,
    pub phone_verified: bool,
    pub role: String,
    sub: String,
    // add more fields as needed
}

#[derive(Debug, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub exp: usize,
    pub user_metadata:  MetadataSchema,
}

pub struct AuthUser(pub Claims);

impl FromRequest for AuthUser {
    type Error = Error;
    type Future = BoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let jwt_secret = req
            .app_data::<web::Data<AppState>>()
            .map(|data| data.jwt_secret.clone());

        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .map(|v| v.trim_start_matches("Bearer ").to_string());

        async move {
            let token = auth_header.ok_or_else(|| ErrorUnauthorized("Falta el header Authorization"))?;
            let secret = jwt_secret.ok_or_else(|| ErrorUnauthorized("No se pudo obtener jwt_secret"))?;

            let mut validation = Validation::new(Algorithm::HS256);
            validation.validate_aud = false; // Disable audience validation for simplicity
            let key = DecodingKey::from_secret(secret.as_bytes());

            let decoded = decode::<Claims>(&token, &key, &validation)
                .map_err(|e| {
                    println!("ERROR en JWT decode: {:?}", e);
                    ErrorUnauthorized(format!("Token inválido o expirado: {}", e))
                })?;


            Ok(AuthUser(decoded.claims))
        }
            .boxed()
    }
}
