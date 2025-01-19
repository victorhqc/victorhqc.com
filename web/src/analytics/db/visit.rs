use super::path::{DBPath, Error as DBPathError};
use crate::analytics::{path::Path, visit::Visit};
use core_victorhqc_com::models::Timestamp;
use snafu::prelude::*;
use sqlx::error::Error as SqlxError;
use sqlx::{FromRow, SqliteConnection};
use time::OffsetDateTime;

#[derive(FromRow)]
pub struct DBVisit {
    pub id: String,
    pub session_id: String,
    pub referer: Option<String>,
    pub created_at: Timestamp,
}

impl Visit {
    pub async fn save(&self, conn: &mut SqliteConnection) -> Result<String, Error> {
        save_visit(conn, self).await
    }
}

async fn save_visit(conn: &mut SqliteConnection, visit: &Visit) -> Result<String, Error> {
    let id = visit.id.clone();

    let (path_id, _) = Path::get_or_create(conn, &visit.path.name)
        .await
        .context(SavePathSnafu)?;

    sqlx::query!(
        "
        INSERT INTO visits (id, session_id, path_id, referer)
        VALUES(?, ?, ?, ?)
        ",
        visit.id,
        visit.session_id,
        path_id,
        visit.referer,
    )
    .execute(&mut *conn)
    .await
    .context(SqlxSnafu)?;

    Ok(id)
}

impl TryFrom<(DBVisit, DBPath)> for Visit {
    type Error = Error;

    fn try_from((value, path): (DBVisit, DBPath)) -> Result<Self, Self::Error> {
        let created_at = {
            let timestamp = value.created_at.0 / 1000;

            OffsetDateTime::from_unix_timestamp(timestamp).context(TimestampSnafu)?
        };

        Ok(Visit {
            id: value.id,
            path: Path { name: path.path },
            session_id: value.session_id,
            referer: value.referer,
            created_at,
        })
    }
}

impl From<&Visit> for DBVisit {
    fn from(value: &Visit) -> DBVisit {
        DBVisit {
            id: value.id.clone(),
            session_id: value.session_id.clone(),
            referer: value.referer.clone(),
            created_at: value.created_at.into(),
        }
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to execute query: {}", source))]
    Sqlx { source: SqlxError },

    #[snafu(display("Failed to parse timestamp: {}", source))]
    Timestamp { source: time::error::ComponentRange },

    #[snafu(display("Failed to save the path: {}", source))]
    SavePath { source: DBPathError },
}
