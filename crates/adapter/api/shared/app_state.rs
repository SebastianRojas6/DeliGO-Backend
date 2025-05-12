use crate::spi::db::db_product_facts_repository::ProductFactsRepository;
use crate::spi::db::db_user_facts_repository::UserFactsRepository;

pub struct AppState {
    pub app_name: String,
    pub product_repository: ProductFactsRepository,
    pub user_repository: UserFactsRepository,
}