pub mod db;

use rocket::serde::Serialize;
use time::OffsetDateTime;

#[derive(Clone, Debug, Serialize)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted: bool,
}
