mod database;
mod dto;
mod routes;
mod util;

use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use crate::util::app_error::AppResult;

#[cfg(feature = "sqlx-postgres")]
use crate::util::app_db::create_db_pool;

#[cfg(feature = "sqlx-sqlite")]
use crate::util::app_sqlite::create_sqlite_pool;

pub async fn run(db_uri: &str, bind_addr: &str) -> AppResult<()> {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            format!("{}=error,tower_http=error", env!("CARGO_CRATE_NAME")).into()
        }))
        .with(tracing_subscriber::fmt::layer())
        .init();

    #[cfg(feature = "sqlx-sqlite")]
    let db_conn = create_sqlite_pool(db_uri).await?;
    #[cfg(feature = "sqlx-postgres")]
    let db_conn = create_db_pool(db_uri).await?;

    let app = routes::create_routes(db_conn).await;
    let listener = TcpListener::bind(bind_addr).await?;

    axum::serve(listener, app).await?;

    Ok(())
}
