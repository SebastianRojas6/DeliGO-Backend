use std::sync::Arc;
use admin::crud_product::domain::repository::ProductAbstractRepository;
use admin::crud_product::infrastructure::spi::db::db_product_facts_repository::DbProductFactsRepository;
use shared::config::connect_to_supabase;

/*

Importanciones

use crate::enrollment::infrastructure::use_cases::repository::SupabaseEnrollmentRepository;
use crate::crud_enrollment::infrastructure::use_cases::repository::SupabaseCrudEnrollmentRepository;
use crate::availability::infrastructure::use_cases::repository::SupabaseAvailabilityRepository;

*/
#[derive(Clone)]
pub struct AppState {
    pub jwt_secret: String,
    pub products_repo: Arc<dyn ProductAbstractRepository>,
    
    // pub crud_repo: Arc<SupabaseCrudEnrollmentRepository>,
    // pub enrollment_repo: Arc<SupabaseEnrollmentRepository>,
    // pub availability_repo: Arc<SupabaseAvailabilityRepository>,
}

pub async fn init_state() -> Result<AppState, Box<dyn std::error::Error + Send + Sync>> {
    let _db = connect_to_supabase().await?;
    let supabase_jwt_secret = std::env::var("SUPABASE_JWT_SECRET")
        .expect("Falta la variable SUPABASE_JWT_SECRET");
    
    let products_repo = Arc::new(DbProductFactsRepository::new(_db.clone()));

    Ok(AppState {
        jwt_secret: supabase_jwt_secret,
        products_repo,
        // crud_repo: Arc::new(SupabaseCrudEnrollmentRepository::new(db.clone())),
        // enrollment_repo: Arc::new(SupabaseEnrollmentRepository { db: db.clone() }),
        // availability_repo: Arc::new(SupabaseAvailabilityRepository { db }),
    })
}
