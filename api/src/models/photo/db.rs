use super::Photo;
use crate::models::{FileType, Timestamp};
use snafu::prelude::*;
use sqlx::error::Error as SqlxError;
use sqlx::{FromRow, SqlitePool};
use std::str::FromStr;
use time::OffsetDateTime;

#[derive(FromRow)]
struct DBPhoto {
    id: String,
    src: String,
    filename: String,
    rating: i64,
    city: Option<String>,
    filetype: String,
    date_taken: Option<Timestamp>,
    exif_meta_id: String,
    created_at: Timestamp,
    updated_at: Timestamp,
    deleted: bool,
}

#[derive(FromRow)]
struct DBTagPhoto {
    tag_id: String,
    id: String,
    src: String,
    filename: String,
    rating: i64,
    city: Option<String>,
    filetype: String,
    date_taken: Option<Timestamp>,
    exif_meta_id: String,
    created_at: Timestamp,
    updated_at: Timestamp,
    deleted: bool,
}

impl Photo {
    pub async fn find_by_id(pool: &SqlitePool, id: &str) -> Result<Photo, Error> {
        find_by_id(pool, id).await
    }

    pub async fn find_by_tag_ids(
        pool: &SqlitePool,
        ids: &Vec<String>,
    ) -> Result<Vec<(String, Photo)>, Error> {
        find_by_tag_ids(pool, ids).await
    }

    pub async fn find_all(pool: &SqlitePool) -> Result<Vec<Photo>, Error> {
        find_all(pool).await
    }
}

async fn find_by_id(pool: &SqlitePool, id: &str) -> Result<Photo, Error> {
    let photo = sqlx::query_as!(
        DBPhoto,
        r#"
    SELECT
        id,
        src,
        filename,
        rating,
        filetype,
        date_taken as "date_taken: Timestamp",
        city,
        exif_meta_id,
        created_at as "created_at: Timestamp",
        updated_at as "updated_at: Timestamp",
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

    Ok(photo.try_into()?)
}

async fn find_all(pool: &SqlitePool) -> Result<Vec<Photo>, Error> {
    let photos = sqlx::query_as!(
        DBPhoto,
        r#"
    SELECT
        id,
        src,
        filename,
        rating,
        filetype,
        date_taken as "date_taken: Timestamp",
        city,
        exif_meta_id,
        created_at as "created_at: Timestamp",
        updated_at as "updated_at: Timestamp",
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

    let photos: Vec<Photo> = photos.into_iter().map(|p| p.try_into().unwrap()).collect();

    Ok(photos)
}

async fn find_by_tag_ids(
    pool: &SqlitePool,
    ids: &Vec<String>,
) -> Result<Vec<(String, Photo)>, Error> {
    let params = format!("?{}", ", ?".repeat(ids.len() - 1));

    let query = format!(
        r#"
    SELECT
        tag_id,
        p.id,
        src,
        filename,
        rating,
        filetype,
        date_taken,
        city,
        exif_meta_id,
        p.created_at,
        p.updated_at,
        p.deleted
    FROM
        photos as p
    JOIN
        photo_tags as pt ON pt.photo_id = p.id
    WHERE
        pt.tag_id IN ( { } )
        AND deleted = false
    "#,
        params
    );

    let mut query = sqlx::query_as::<_, DBTagPhoto>(&query);

    for id in ids {
        query = query.bind(id);
    }

    let photos = query.fetch_all(pool).await.context(SqlxSnafu)?;

    let photos: Vec<(String, Photo)> = photos
        .into_iter()
        .map(|p| {
            (
                p.tag_id,
                DBPhoto {
                    id: p.id,
                    src: p.src,
                    filename: p.filename,
                    rating: p.rating,
                    filetype: p.filetype,
                    date_taken: p.date_taken,
                    city: p.city,
                    exif_meta_id: p.exif_meta_id,
                    created_at: p.created_at,
                    updated_at: p.updated_at,
                    deleted: p.deleted,
                },
            )
        })
        .map(|(id, t)| (id, t.try_into().unwrap()))
        .collect();

    Ok(photos)
}

impl TryFrom<DBPhoto> for Photo {
    type Error = Error;

    fn try_from(photo: DBPhoto) -> Result<Self, Error> {
        let filetype = FileType::from_str(&photo.filetype).context(FileTypeSnafu)?;

        let date_taken = if let Some(v) = photo.date_taken {
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
            let timestamp = photo.created_at.0 / 1000;

            OffsetDateTime::from_unix_timestamp(timestamp).context(TimestampSnafu)?
        };

        let updated_at = {
            // Time is in milliseconds
            let timestamp = photo.updated_at.0 / 1000;

            OffsetDateTime::from_unix_timestamp(timestamp).context(TimestampSnafu)?
        };

        Ok(Photo {
            id: photo.id,
            src: photo.src,
            filename: photo.filename,
            rating: photo.rating as i8,
            filetype,
            date_taken,
            city: photo.city,
            exif_meta_id: photo.exif_meta_id,
            created_at,
            updated_at,
            deleted: photo.deleted,
        })
    }
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
