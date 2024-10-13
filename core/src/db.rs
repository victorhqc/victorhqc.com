use snafu::prelude::*;
use sqlx::SqlitePool;

pub async fn get_pool(url: &str) -> Result<SqlitePool, Error> {
    let pool = SqlitePool::connect(url).await.context(ConnectSnafu)?;

    Ok(pool)
}

pub async fn migrate(pool: &SqlitePool) -> Result<(), Error> {
    sqlx::migrate!().run(pool).await.context(MigrationsSnafu)?;

    Ok(())
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to run migrations: {}", source))]
    Migrations { source: sqlx::migrate::MigrateError },

    #[snafu(display("Failed to connect to Pool: {}", source))]
    Connect { source: sqlx::Error },
}
