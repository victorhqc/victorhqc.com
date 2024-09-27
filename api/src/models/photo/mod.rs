pub mod db;

use crate::models::FileType;
use rocket::serde::Serialize;
use time::{Date, OffsetDateTime};

#[derive(Clone, Debug, Serialize)]
pub struct Photo {
    pub id: String,
    pub src: String,
    pub filename: String,
    pub rating: i8,
    pub filetype: FileType,
    pub date_taken: Option<Date>,
    pub city: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted: bool,
}
