use std::sync::Arc;

use admin::crud_product::domain::repository::ProductAbstractRepository;
use admin::crud_product::infrastructure::spi::db::db_product_facts_repository::DbProductFactsRepository;

use shared::config::connect_to_supabase;

use register_login::credential_validation::infrastructure::repository::SeaOrmUserCredentialRepository;
use orders_billing::payment_record::domain::repository::OrdersBillingRepository;
use orders_billing::payment_record::infrastructure::repository::SupabaseOrdersBillingRepository;
use admin::crud_delivery_man::domain::repository::DeliveryManAbstractRepository;
use admin::crud_delivery_man::infrastructure::spi::db_delivery_facts_repository::DbDeliveryFactsRepository;
use register_login::credential_validation::domain::repository::UserCredentialRepository;

#[derive(Clone)]
pub struct AppState {
    pub jwt_secret: String,
    pub products_repo: Arc<dyn ProductAbstractRepository>,
    pub credential_repo: Arc<dyn UserCredentialRepository>,
    pub delivery_man_repo: Arc<dyn DeliveryManAbstractRepository>,
    pub orders_billing_repo: Arc<dyn OrdersBillingRepository>,
}

pub async fn init_state() -> Result<AppState, Box<dyn std::error::Error + Send + Sync>> {
    let db = connect_to_supabase().await?;

    let supabase_jwt_secret = std::env::var("SUPABASE_JWT_SECRET")
        .expect("Falta la variable SUPABASE_JWT_SECRET");

    let products_repo = Arc::new(DbProductFactsRepository::new(db.clone()));
    let delivery_man_repo = Arc::new(DbDeliveryFactsRepository::new(db.clone()));
    let credential_repo = Arc::new(SeaOrmUserCredentialRepository { db: db.clone() });
    let orders_billing_repo: Arc<SupabaseOrdersBillingRepository> = Arc::new(SupabaseOrdersBillingRepository::new(db.clone()));

    Ok(AppState {
        jwt_secret: supabase_jwt_secret,
        products_repo,
        delivery_man_repo,
        credential_repo,
        orders_billing_repo,
    })
}
