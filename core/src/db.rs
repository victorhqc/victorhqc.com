use snafu::prelude::*;
use sqlx::SqlitePool;

pub async fn migrate(pool: &SqlitePool) -> Result<(), Error> {
    sqlx::migrate!()
        .run(pool)
        .await
        .context(MigrationsSnafu)?;

    Ok(())
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to run migrations: {}", source))]
    Migrations { source: sqlx::migrate::MigrateError },
}
