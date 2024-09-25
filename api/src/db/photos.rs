use crate::models::{FileType, Photo};
use snafu::prelude::*;
use sqlx::error::Error as SqlxError;
use sqlx::{FromRow, SqlitePool};
use std::str::FromStr;
use time::OffsetDateTime;

#[derive(FromRow)]
pub struct DBPhoto {
    id: String,
    src: String,
    filename: String,
    rating: i64,
    city: Option<String>,
    filetype: String,
    date_taken: Option<super::Timestamp>,
    exif_meta_id: String,
    created_at: super::Timestamp,
    updated_at: super::Timestamp,
    deleted: bool,
}

impl Photo {
    pub fn from_db(row: DBPhoto) -> Result<Self, Error> {
        let filetype = FileType::from_str(&row.filetype).context(FileTypeSnafu)?;

        let date_taken = if let Some(v) = row.date_taken {
            // Time is in milliseconds
            let timestamp = v.0 / 1000;
            let datetime =
                OffsetDateTime::from_unix_timestamp(timestamp).context(TimestampSnafu)?;

            Some(datetime.date())
        } else {
            None
        };

        let created_at = {
            // Time is in milliseconds
            let timestamp = row.created_at.0 / 1000;

            OffsetDateTime::from_unix_timestamp(timestamp).context(TimestampSnafu)?
        };

        let updated_at = {
            // Time is in milliseconds
            let timestamp = row.updated_at.0 / 1000;

            OffsetDateTime::from_unix_timestamp(timestamp).context(TimestampSnafu)?
        };

        Ok(Photo {
            id: row.id,
            src: row.src,
            filename: row.filename,
            rating: row.rating as i8,
            filetype,
            date_taken,
            city: row.city,
            exif_meta_id: row.exif_meta_id,
            created_at,
            updated_at,
            deleted: row.deleted,
        })
    }
}

pub async fn get_photo_by_id(pool: &SqlitePool, id: &str) -> Result<Photo, Error> {
    let row = sqlx::query_as!(
        DBPhoto,
        r#"
    SELECT
        id,
        src,
        filename,
        rating,
        filetype,
        date_taken as "date_taken: crate::db::Timestamp",
        city,
        exif_meta_id,
        created_at as "created_at: crate::db::Timestamp",
        updated_at as "updated_at: crate::db::Timestamp",
        deleted
    FROM
        photos
    WHERE
        deleted = false
        AND id = ?
    ORDER BY
        created_at DESC
    "#,
        id
    )
    .fetch_one(pool)
    .await
    .context(SqlxSnafu)?;

    let photo = Photo::from_db(row)?;

    Ok(photo)
}

pub async fn get_all_photos(pool: &SqlitePool) -> Result<Vec<Photo>, Error> {
    let photos = sqlx::query_as!(
        DBPhoto,
        r#"
    SELECT
        id,
        src,
        filename,
        rating,
        filetype,
        date_taken as "date_taken: crate::db::Timestamp",
        city,
        exif_meta_id,
        created_at as "created_at: crate::db::Timestamp",
        updated_at as "updated_at: crate::db::Timestamp",
        deleted
    FROM
        photos AS p
    WHERE
        deleted = false
    ORDER BY
        created_at DESC
    "#
    )
    .fetch_all(pool)
    .await
    .context(SqlxSnafu)?;

    let photos = photos
        .into_iter()
        .map(|p| Photo::from_db(p).unwrap())
        .collect();

    Ok(photos)
}

pub async fn get_photos_by_ids(pool: &SqlitePool, ids: &Vec<String>) -> Result<Vec<Photo>, Error> {
    let params = format!("?{}", ", ?".repeat(ids.len() - 1));

    let query = format!(
        r#"
    SELECT
        id,
        src,
        filename,
        rating,
        filetype,
        date_taken as "date_taken: crate::db::Timestamp",
        city,
        exif_meta_id,
        created_at as "created_at: crate::db::Timestamp",
        updated_at as "updated_at: crate::db::Timestamp",
        deleted
    FROM
        photos
    WHERE
        deleted = false
        AND id IN ( { } )
    "#,
        params
    );

    let mut query = sqlx::query_as::<_, DBPhoto>(&query);

    for id in ids {
        query = query.bind(id);
    }

    let photos = query.fetch_all(pool).await.context(SqlxSnafu)?;

    let photos = photos
        .into_iter()
        .map(|p| Photo::from_db(p).unwrap())
        .collect();

    Ok(photos)
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to execute query: {:?}", source))]
    Sqlx { source: SqlxError },

    #[snafu(display("Failed to parse FileType {:?}", source))]
    FileType { source: strum::ParseError },

    #[snafu(display("Failed to parse timestamp: {:?}", source))]
    Timestamp { source: time::error::ComponentRange },
}
