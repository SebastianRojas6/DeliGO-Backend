use std::time::Duration;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};


pub struct DbConnectionProvider {
    pub connection: DatabaseConnection,
}
impl DbConnectionProvider {
    pub async fn new() -> Self {
        let db_url = "postgres://pguser:pgpassword@localhost:5432/deligo?currentSchema=my_schema";
        let mut opt = ConnectOptions::new(db_url);
        opt
            .max_connections(20)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(10))
            .idle_timeout(Duration::from_secs(30))
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Debug);

        let connection = Database::connect(opt).await.expect("failed to connect to DB");
        Self { connection }
    }

    pub fn get_connection(&self) -> &DatabaseConnection {
        &self.connection
    }
}