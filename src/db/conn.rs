use std::{
    path::PathBuf,
    str::FromStr,
    sync::{Arc, LazyLock},
};

use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait as _;
use tokio::sync::RwLock;

use super::migrations::Migrator;

static DB_POOL: LazyLock<Arc<RwLock<Option<DatabaseConnection>>>> =
    LazyLock::new(|| Arc::new(RwLock::new(None)));

pub async fn create_sqlite_db_file(database_url: &str) -> Result<(), anyhow::Error> {
    if database_url == "sqlite::memory:" {
        return Ok(());
    }
    let db_file: PathBuf = PathBuf::from_str(database_url.split("//").last().ok_or(
        anyhow::anyhow!("Invalid database url, use 'sqlite://path/to/database.db'"),
    )?)?;

    if db_file.parent().is_none() {
        tokio::fs::create_dir_all(db_file.parent().unwrap())
            .await
            .map_err(|e| {
                anyhow::anyhow!(
                    "Failed to create parent directory when create sqlite db file: {}",
                    e
                )
            })?;
    };

    if !db_file.exists() {
        tokio::fs::File::create(db_file).await?;
    }
    Ok(())
}

pub async fn set_db(database_url: &str) -> Result<(), anyhow::Error> {
    close_db().await?;

    if database_url.starts_with("sqlite") {
        create_sqlite_db_file(database_url).await?;
    }

    let db_connection = Database::connect(database_url).await?;

    Migrator::up(&db_connection, None).await?;

    let mut db_pool = DB_POOL.write().await;

    *db_pool = Some(db_connection);
    Ok(())
}

pub async fn get_db() -> anyhow::Result<DatabaseConnection> {
    DB_POOL
        .read()
        .await
        .clone()
        .ok_or(anyhow::anyhow!("Database is not inited"))
}

pub async fn try_get_db() -> Option<DatabaseConnection> {
    DB_POOL.read().await.clone()
}

pub async fn close_db() -> Result<(), anyhow::Error> {
    let mut db = DB_POOL.write().await;
    if let Some(db_conn) = db.clone() {
        db_conn.close().await?;
    }

    *db = None;

    Ok(())
}

pub async fn clear_db() -> Result<(), anyhow::Error> {
    let db = get_db().await?;
    Migrator::down(&db, None).await?;
    Ok(())
}

pub async fn reinit_db() -> Result<(), anyhow::Error> {
    let db = get_db().await?;
    Migrator::down(&db, None).await?;
    Migrator::up(&db, None).await?;
    Ok(())
}
