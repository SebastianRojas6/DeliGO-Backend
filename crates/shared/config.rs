use sea_orm::{Database, DatabaseConnection};
use std::env;

pub async fn connect_to_supabase() -> Result<DatabaseConnection, Box<dyn std::error::Error + Send + Sync>> {
    let database_url = env::var("SUPABASE_URL")?;
    let db = Database::connect(&database_url).await?;
    Ok(db)
}