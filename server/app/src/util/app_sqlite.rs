use std::str::FromStr;

use crate::database::{tasks, users};
use crate::util::app_db::create_db_pool;
use crate::util::app_error::AppResult;
use crate::util::app_utils::crypt_hash;
use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ConnectionTrait, DatabaseConnection, DbBackend, Schema, SqlxSqliteConnector,
};
use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{Sqlite, SqlitePool};

pub async fn create_sqlite_pool(db_uri: &str) -> AppResult<DatabaseConnection> {
    if Sqlite::database_exists(db_uri).await? {
        Ok(create_db_pool(db_uri).await?)
    } else {
        {
            let pool = SqlxSqliteConnector::from_sqlx_sqlite_pool(
                SqlitePool::connect_with(
                    SqliteConnectOptions::from_str(db_uri)?.create_if_missing(true),
                )
                .await?,
            );

            sqlite_scheme_init(&pool).await?;
            create_test_data(&pool).await?;
        }

        Ok(create_db_pool(db_uri).await?)
    }
}

async fn sqlite_scheme_init(db: &DatabaseConnection) -> AppResult<()> {
    let schema = Schema::new(DbBackend::Sqlite);

    let stmt = schema.create_table_from_entity(users::Entity);
    db.execute(db.get_database_backend().build(&stmt)).await?;

    let stmt = schema.create_table_from_entity(tasks::Entity);
    db.execute(db.get_database_backend().build(&stmt)).await?;

    Ok(())
}

async fn create_test_data(db: &DatabaseConnection) -> AppResult<()> {
    let db_user = users::ActiveModel {
        username: Set("testuser".to_owned()),
        password: Set(crypt_hash("1234".to_owned())?),
        ..Default::default()
    }
    .save(db)
    .await?;

    tasks::ActiveModel {
        priority: Set(None),
        title: Set("my deleted task".to_owned()),
        deleted_at: Set(Some(Utc::now().fixed_offset().to_rfc2822())),
        user_id: Set(Some(db_user.id.unwrap())),
        ..Default::default()
    }
    .save(db)
    .await?;

    tasks::ActiveModel {
        priority: Set(Some("A".to_owned())),
        title: Set("I am a task, you can complete me by checking the box".to_owned()),
        description: Set(Some("This is my description".to_owned())),
        is_default: Set(Some(true)),
        ..Default::default()
    }
    .save(db)
    .await?;

    tasks::ActiveModel {
        priority: Set(Some("B".to_owned())),
        title: Set("See my details for by clicking me".to_owned()),
        description: Set(Some("My description can be changed".to_owned())),
        is_default: Set(Some(true)),
        ..Default::default()
    }
    .save(db)
    .await?;

    Ok(())
}
