use super::path::Path;
use super::session::Session;
use serde::Serialize;
use snafu::prelude::*;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize)]
pub struct Visit {
    pub id: String,
    pub session_id: String,
    pub path: Path,
    pub referer: Option<String>,
    pub created_at: OffsetDateTime,
}

impl Visit {
    pub fn new(session: &Session, path: String, referer: Option<String>) -> Result<Self, Error> {
        let id = Uuid::new_v4().to_string();

        let now = OffsetDateTime::now_utc().unix_timestamp();
        let created_at = OffsetDateTime::from_unix_timestamp(now).context(InvalidDateSnafu)?;

        Ok(Visit {
            id,
            session_id: session.id.clone(),
            path: Path { name: path },
            referer,
            created_at,
        })
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to crate date: {}", source))]
    InvalidDate { source: time::error::ComponentRange },
}
