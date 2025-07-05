use crate::infrastructure::querys::{product_query::ProductQuery, user_query::UserQuery};
use std::sync::Arc;

pub struct AppState {
    pub user_repo: Arc<UserQuery>,
    pub product_repo: Arc<ProductQuery>,
}
