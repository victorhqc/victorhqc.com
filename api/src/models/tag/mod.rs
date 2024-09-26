pub mod db;

use time::OffsetDateTime;
use rocket::serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted: bool,
}
