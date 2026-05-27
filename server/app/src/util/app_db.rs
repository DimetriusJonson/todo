use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

pub async fn create_db_pool(db_uri: &str) -> Result<DatabaseConnection, DbErr> {
    Database::connect(
        ConnectOptions::new(db_uri)
            .max_connections(3)
            .min_connections(1)
            .connect_timeout(Duration::from_secs(30))
            .idle_timeout(Duration::from_secs(600))
            .max_lifetime(Duration::from_secs(600))
            .sqlx_logging(true)
            .clone(),
    )
    .await
}
