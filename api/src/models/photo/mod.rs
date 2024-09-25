mod db;

use crate::models::FileType;
use rocket::serde::Serialize;
use snafu::prelude::*;
use sqlx::SqlitePool;
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
    pub exif_meta_id: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted: bool,
}

impl Photo {
    pub async fn find_by_id(pool: &SqlitePool, id: &str) -> Result<Photo, Error> {
        db::get_photo_by_id(pool, id).await.context(DBSnafu)
    }

    pub async fn find_by_ids(pool: &SqlitePool, ids: &Vec<String>) -> Result<Vec<Photo>, Error> {
        db::get_photos_by_ids(pool, ids).await.context(DBSnafu)
    }

    pub async fn find_all(pool: &SqlitePool) -> Result<Vec<Photo>, Error> {
        db::get_all_photos(pool).await.context(DBSnafu)
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to get photos: {:?}", source))]
    DB { source: db::Error },
}
