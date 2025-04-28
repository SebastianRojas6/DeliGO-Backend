use std::time::Duration;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

async fn connect() -> DatabaseConnection {
    let db_url = "mysql://root:password@localhost:3306/test_db";
    let mut opt = ConnectOptions::new(db_url);
    opt
        .max_connections(20)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(30))
        .sqlx_logging(true) 
        .sqlx_logging_level(log::LevelFilter::Debug);

    Database::connect(opt).await.expect("failed to connect to DB")
}