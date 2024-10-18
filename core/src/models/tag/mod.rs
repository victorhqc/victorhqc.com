pub mod db;

use serde::Serialize;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted: bool,
}

impl Tag {
    pub fn new(name: String) -> Self {
        let id = Uuid::new_v4().to_string();
        
        let now = OffsetDateTime::now_utc().unix_timestamp();
        let created_at = OffsetDateTime::from_unix_timestamp(now).unwrap();
        let updated_at = OffsetDateTime::from_unix_timestamp(now).unwrap();
        
        Tag {
            id,
            name,
            created_at,
            updated_at,
            deleted: false,
        }
    }
}

