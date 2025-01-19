use crate::analytics::session::Session;
use core_victorhqc_com::models::Timestamp;
use log::debug;
use snafu::prelude::*;
use sqlx::error::Error as SqlxError;
use sqlx::{FromRow, SqliteConnection};
use time::OffsetDateTime;

#[derive(FromRow)]
pub struct DBSession {
    pub id: String,
    pub user_agent: Option<String>,
    pub created_at: Timestamp,
    pub last_activity_at: Timestamp,
}

impl Session {
    pub async fn create_or_update(&self, conn: &mut SqliteConnection) -> Result<(), Error> {
        match get_by_id(conn, &self.id).await? {
            Some(_) => update_last_activity(conn, &self.id).await,
            None => create_session(conn, self).await.map(|_| ()),
        }
    }
}

async fn get_by_id(conn: &mut SqliteConnection, id: &str) -> Result<Option<DBSession>, Error> {
    let maybe = sqlx::query_as!(
        DBSession,
        "
        SELECT * FROM sessions WHERE id = ?
        ",
        id
    )
    .fetch_optional(&mut *conn)
    .await
    .context(SqlxSnafu)?;

    Ok(maybe)
}

async fn update_last_activity(conn: &mut SqliteConnection, id: &str) -> Result<(), Error> {
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let last_activity_at = OffsetDateTime::from_unix_timestamp(now).context(TimestampSnafu)?;

    sqlx::query!(
        "
        UPDATE sessions
        SET last_activity_at = ?
        WHERE id = ?
        ",
        last_activity_at,
        id
    )
    .execute(&mut *conn)
    .await
    .context(SqlxSnafu)?;

    Ok(())
}

async fn create_session(conn: &mut SqliteConnection, session: &Session) -> Result<String, Error> {
    let id = session.id.clone();

    debug!("Storing session: {:?}", session);

    sqlx::query!(
        "
        INSERT INTO sessions (id, user_agent, last_activity_at)
        VALUES (?, ?, ?)
        ",
        session.id,
        session.user_agent,
        session.last_activity_at
    )
    .execute(&mut *conn)
    .await
    .context(SqlxSnafu)?;

    Ok(id)
}

impl TryFrom<DBSession> for Session {
    type Error = Error;

    fn try_from(value: DBSession) -> Result<Self, Self::Error> {
        let created_at = {
            let timestamp = value.created_at.0 / 1000;

            OffsetDateTime::from_unix_timestamp(timestamp).context(TimestampSnafu)?
        };

        let last_activity_at = {
            let timestamp = value.last_activity_at.0 / 1000;

            OffsetDateTime::from_unix_timestamp(timestamp).context(TimestampSnafu)?
        };

        Ok(Session {
            id: value.id,
            user_agent: value.user_agent,
            created_at,
            last_activity_at,
        })
    }
}

impl From<&Session> for DBSession {
    fn from(value: &Session) -> Self {
        DBSession {
            id: value.id.clone(),
            user_agent: value.user_agent.clone(),
            created_at: value.created_at.into(),
            last_activity_at: value.last_activity_at.into(),
        }
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to execute query: {}", source))]
    Sqlx { source: SqlxError },

    #[snafu(display("Failed to parse timestamp: {}", source))]
    Timestamp { source: time::error::ComponentRange },
}
