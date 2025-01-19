use super::record::UniqueId;
use serde::Serialize;
use snafu::prelude::*;
use time::OffsetDateTime;

#[derive(Clone, Debug, Serialize)]
pub struct Session {
    pub id: String,
    pub user_agent: Option<String>,
    pub created_at: OffsetDateTime,
    pub last_activity_at: OffsetDateTime,
}

impl Session {
    pub fn new(id: UniqueId, user_agent: Option<String>) -> Result<Self, Error> {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        let created_at = OffsetDateTime::from_unix_timestamp(now).context(InvalidDateSnafu)?;

        Ok(Session {
            id: id.0,
            user_agent,
            created_at,
            last_activity_at: created_at,
        })
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to crate date: {}", source))]
    InvalidDate { source: time::error::ComponentRange },
}
