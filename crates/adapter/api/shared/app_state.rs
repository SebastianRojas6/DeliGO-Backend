use crate::spi::db::db_product_facts_repository::ProductFactsRepository;

pub struct AppState {
    pub app_name: String,
    pub product_repository: ProductFactsRepository,
}