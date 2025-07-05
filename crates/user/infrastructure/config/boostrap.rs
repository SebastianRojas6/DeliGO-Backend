use super::app_state::AppState;
use crate::infrastructure::querys::{product_query::ProductQuery, user_query::UserQuery};
use std::sync::Arc;

pub async fn bootstrap_user() -> Result<AppState, String> {
    let user_repo = Arc::new(UserQuery::new().await?);
    let product_repo = Arc::new(ProductQuery::new().await?);

    // Crear estado de aplicación
    Ok(AppState { user_repo, product_repo })
}
