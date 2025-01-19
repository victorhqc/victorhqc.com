use crate::analytics::path::Path;
use snafu::prelude::*;
use sqlx::error::Error as SqlxError;
use sqlx::{FromRow, SqliteConnection};

#[derive(FromRow)]
pub struct DBPath {
    pub id: Option<i64>,
    pub path: String,
}

impl Path {
    pub async fn get_or_create(
        conn: &mut SqliteConnection,
        name: &str,
    ) -> Result<(i64, Self), Error> {
        let maybe: (i64, Path) = match Path::get_by_value(conn, name).await {
            Ok(p) => p,
            Err(e) => match e {
                Error::Sqlx { source } => match source {
                    SqlxError::RowNotFound => {
                        let id = save_path(conn, name).await?;

                        (
                            id,
                            Path {
                                name: String::from(name),
                            },
                        )
                    }
                    _ => return Err(Error::Sqlx { source }),
                },
            },
        };

        Ok(maybe)
    }

    pub async fn get_by_value(
        conn: &mut SqliteConnection,
        value: &str,
    ) -> Result<(i64, Self), Error> {
        let path = get_by_value(conn, value).await?;

        match path {
            Some(p) => {
                let id: i64 = p.id.unwrap_or(0);

                p.try_into().map(|r| (id, r))
            }
            None => Err(Error::Sqlx {
                source: sqlx::Error::RowNotFound,
            }),
        }
    }
}

async fn get_by_value(conn: &mut SqliteConnection, value: &str) -> Result<Option<DBPath>, Error> {
    sqlx::query_as!(
        DBPath,
        "
        SELECT
            id,
            path
        FROM paths
        WHERE path = ?
        ",
        value
    )
    .fetch_optional(conn)
    .await
    .context(SqlxSnafu)
}

async fn save_path(conn: &mut SqliteConnection, value: &str) -> Result<i64, Error> {
    let id = sqlx::query!(
        "
        INSERT INTO paths (path)
        VALUES (?)
        ",
        value,
    )
    .execute(&mut *conn)
    .await
    .context(SqlxSnafu)?
    .last_insert_rowid();

    Ok(id)
}

impl TryFrom<DBPath> for Path {
    type Error = Error;

    fn try_from(value: DBPath) -> Result<Self, Self::Error> {
        Ok(Path { name: value.path })
    }
}

impl From<&Path> for DBPath {
    fn from(value: &Path) -> Self {
        DBPath {
            id: None,
            path: value.name.clone(),
        }
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to execute query: {}", source))]
    Sqlx { source: SqlxError },
}
