use shared::config::connect_to_supabase;

use std::sync::Arc;

use register_login::credential_validation::infrastructure::repository::SeaOrmUserCredentialRepository;
use register_login::credential_validation::domain::repository::UserCredentialRepository;

#[derive(Clone)]
pub struct AppState {
    pub jwt_secret: String,
    pub credential_repo: Arc<dyn UserCredentialRepository>,

}

pub async fn init_state() -> Result<AppState, Box<dyn std::error::Error + Send + Sync>> {
    let db = connect_to_supabase().await?;
    let supabase_jwt_secret = std::env::var("SUPABASE_JWT_SECRET")
        .expect("Falta la variable SUPABASE_JWT_SECRET");

    let credential_repo = Arc::new(SeaOrmUserCredentialRepository { db });

    Ok(AppState {
        jwt_secret: supabase_jwt_secret,
        credential_repo,
    })
}
