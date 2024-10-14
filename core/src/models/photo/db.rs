use super::{FileType, Photo};
use crate::models::Timestamp;
use snafu::prelude::*;
use sqlx::error::Error as SqlxError;
use sqlx::{FromRow, SqliteConnection, SqlitePool};
use std::str::FromStr;
use time::OffsetDateTime;
use uuid::Error as UuidError;

#[derive(FromRow)]
struct DBPhoto {
    id: String,
    title: String,
    filename: String,
    filetype: String,
    created_at: Timestamp,
    updated_at: Timestamp,
    deleted: bool,
}

#[derive(FromRow)]
struct DBTagPhoto {
    tag_id: String,
    id: String,
    title: String,
    filename: String,
    filetype: String,
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

    pub async fn save(&self, pool: &mut SqliteConnection) -> Result<String, Error> {
        let photo: DBPhoto = self.into();
        insert(pool, photo).await
    }
}

async fn find_by_id(pool: &SqlitePool, id: &str) -> Result<Photo, Error> {
    // TODO: Move back to macro. Fails to compile in IDE because fails to find DB
    let photo = sqlx::query_as::<_, DBPhoto>(
        r#"
    SELECT
        id,
        title,
        filename,
        filetype,
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
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .context(SqlxSnafu)?;

    photo.try_into()
}

async fn find_all(pool: &SqlitePool) -> Result<Vec<Photo>, Error> {
    // TODO: Move back to macro. Fails to compile in IDE because fails to find DB
    let photos = sqlx::query_as::<_, DBPhoto>(
        r#"
    SELECT
        id,
        title,
        filename,
        filetype,
        created_at,
        updated_at,
        deleted
    FROM
        photos AS p
    WHERE
        deleted = false
    ORDER BY
        created_at DESC
    "#,
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
        title,
        filename,
        filetype,
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
                    title: p.title,
                    filename: p.filename,
                    filetype: p.filetype,
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

async fn insert(conn: &mut SqliteConnection, photo: DBPhoto) -> Result<String, Error> {
    let id = photo.id.clone();

    sqlx::query(
        r#"
    INSERT INTO photos (id, title, filename, filetype, created_at, updated_at, deleted)
    VALUES (?, ?, ?, ?, ?, ?, ?)
    "#,
    )
    .bind(&photo.id)
    .bind(&photo.title)
    .bind(&photo.filename)
    .bind(&photo.filetype)
    .bind(&photo.created_at)
    .bind(&photo.updated_at)
    .bind(photo.deleted)
    .execute(conn)
    .await
    .context(SqlxSnafu)?;

    Ok(id)
}

impl TryFrom<DBPhoto> for Photo {
    type Error = Error;

    fn try_from(photo: DBPhoto) -> Result<Self, Error> {
        let filetype = FileType::from_str(&photo.filetype).context(FileTypeSnafu)?;

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
            title: photo.title,
            filename: photo.filename,
            filetype,
            created_at,
            updated_at,
            deleted: photo.deleted,
        })
    }
}

impl From<&Photo> for DBPhoto {
    fn from(photo: &Photo) -> Self {
        DBPhoto {
            id: photo.id.clone(),
            title: photo.title.clone(),
            filename: photo.filename.clone(),
            filetype: photo.filetype.to_string(),
            created_at: photo.created_at.into(),
            updated_at: photo.created_at.into(),
            deleted: photo.deleted,
        }
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to execute query: {:?}", source))]
    Sqlx { source: SqlxError },

    #[snafu(display("Failed to parse FileType {:?}", source))]
    FileType {
        source: crate::models::photo::str::filetype::Error,
    },

    #[snafu(display("Failed to parse timestamp: {:?}", source))]
    Timestamp { source: time::error::ComponentRange },

    #[snafu(display("Invalid Uuid: {:?}", source))]
    Uuid { source: UuidError },
}
