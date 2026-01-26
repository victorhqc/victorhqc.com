use super::{FileType, Photo};
use crate::models::{
    Timestamp,
    photo::Orientation,
    tag::{Tag, db::Error as TagDbError},
};
use snafu::prelude::*;
use sqlx::error::Error as SqlxError;
use sqlx::{FromRow, SqliteConnection};
use std::path::Path;
use std::str::FromStr;
use time::OffsetDateTime;
use uuid::{Error as UuidError, Uuid};

#[derive(FromRow)]
struct DBPhoto {
    id: String,
    title: String,
    filename: String,
    filetype: String,
    orientation: String,
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
    orientation: String,
    created_at: Timestamp,
    updated_at: Timestamp,
    deleted: bool,
}

impl Photo {
    pub async fn find_by_id(conn: &mut SqliteConnection, id: &str) -> Result<Photo, Error> {
        find_by_id(conn, id).await
    }

    pub async fn find_by_filename(
        conn: &mut SqliteConnection,
        path: &Path,
    ) -> Result<Option<Photo>, Error> {
        find_by_filename(conn, path).await
    }

    pub async fn find_by_tag_ids(
        conn: &mut SqliteConnection,
        ids: &Vec<String>,
        max_results: Option<i32>,
        orientation: Option<Orientation>,
    ) -> Result<Vec<(String, Photo)>, Error> {
        match orientation {
            Some(o) => find_by_tag_ids_and_orientation(conn, ids, &o, max_results).await,
            None => find_by_tag_ids(conn, ids, max_results).await,
        }
    }

    pub async fn find_all(
        conn: &mut SqliteConnection,
        orientation: Option<Orientation>,
    ) -> Result<Vec<Photo>, Error> {
        find_all(conn, &orientation).await
    }

    pub async fn save(&self, conn: &mut SqliteConnection) -> Result<String, Error> {
        let photo: DBPhoto = self.into();
        insert(conn, photo).await
    }

    pub async fn update(&self, conn: &mut SqliteConnection) -> Result<(), Error> {
        let photo: DBPhoto = self.into();

        update(conn, photo).await
    }

    pub async fn save_tags(
        &self,
        conn: &mut SqliteConnection,
        new_tags: &[String],
    ) -> Result<(), Error> {
        for tag in new_tags {
            let tag = Tag::find_by_name_or_create(conn, tag)
                .await
                .context(TagSnafu)?;

            attach_tag(conn, self, &tag).await?;
        }

        Ok(())
    }
}

async fn find_by_id(conn: &mut SqliteConnection, id: &str) -> Result<Photo, Error> {
    let photo = sqlx::query_as!(
        DBPhoto,
        r#"
    SELECT
        id,
        title,
        filename,
        filetype,
        orientation,
        created_at,
        updated_at,
        deleted
    FROM
        photos
    WHERE
        deleted = false
        AND id = ?
    ORDER BY
        created_at ASC
    "#,
        id
    )
    .fetch_one(conn)
    .await
    .context(SqlxSnafu)?;

    photo.try_into()
}

async fn find_by_filename(
    conn: &mut SqliteConnection,
    path: &Path,
) -> Result<Option<Photo>, Error> {
    let filename = path.file_name().unwrap().to_str().unwrap();

    let photo = sqlx::query_as!(
        DBPhoto,
        r#"
    SELECT
        id,
        title,
        filename,
        filetype,
        orientation,
        created_at,
        updated_at,
        deleted
    FROM
        photos
    WHERE
        deleted = false
        AND filename = ?
    ORDER BY
        created_at ASC
    "#,
        filename
    )
    .fetch_optional(conn)
    .await
    .context(SqlxSnafu)?;

    if let Some(photo) = photo.map(|p| p.try_into()) {
        Ok(Some(photo?))
    } else {
        Ok(None)
    }
}

async fn find_all(
    conn: &mut SqliteConnection,
    orientation: &Option<Orientation>,
) -> Result<Vec<Photo>, Error> {
    let photos = match orientation {
        Some(o) => {
            let orientation_str = o.to_string();
            sqlx::query_as!(
                DBPhoto,
                r#"
                SELECT id, title, filename, filetype, orientation,
                       created_at, updated_at, deleted
                FROM photos AS p
                WHERE deleted = false AND orientation = ?
                ORDER BY created_at ASC
                "#,
                orientation_str
            )
            .fetch_all(conn)
            .await
            .context(SqlxSnafu)?
        }
        None => sqlx::query_as!(
            DBPhoto,
            r#"
                SELECT id, title, filename, filetype, orientation,
                       created_at, updated_at, deleted
                FROM photos AS p
                WHERE deleted = false
                ORDER BY created_at ASC
                "#,
        )
        .fetch_all(conn)
        .await
        .context(SqlxSnafu)?,
    };

    let photos: Vec<Photo> = photos.into_iter().map(|p| p.try_into().unwrap()).collect();

    Ok(photos)
}

async fn find_by_tag_ids(
    conn: &mut SqliteConnection,
    ids: &Vec<String>,
    max_results: Option<i32>,
) -> Result<Vec<(String, Photo)>, Error> {
    let limit = max_results.unwrap_or(-1);
    let params = format!("?{}", ", ?".repeat(ids.len() - 1));

    let query = format!(
        r#"
    SELECT
        tag_id,
        p.id,
        title,
        filename,
        filetype,
        orientation,
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
    ORDER BY  p.created_at ASC
    LIMIT {}
    "#,
        params, limit
    );

    let mut query = sqlx::query_as::<_, DBTagPhoto>(&query);

    for id in ids {
        query = query.bind(id);
    }

    let photos = query.fetch_all(conn).await.context(SqlxSnafu)?;

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
                    orientation: p.orientation,
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

async fn find_by_tag_ids_and_orientation(
    conn: &mut SqliteConnection,
    ids: &[String],
    orientation: &Orientation,
    max_results: Option<i32>,
) -> Result<Vec<(String, Photo)>, Error> {
    let limit = max_results.unwrap_or(-1);
    let params = format!("?{}", ", ?".repeat(ids.len() - 1));

    let query = format!(
        r#"
    SELECT
        tag_id,
        p.id,
        title,
        filename,
        filetype,
        orientation,
        p.created_at,
        p.updated_at,
        p.deleted
    FROM
        photos as p
    JOIN
        photo_tags as pt ON pt.photo_id = p.id
    WHERE
        orientation = ?
        AND pt.tag_id IN ( { } )
        AND deleted = false
    ORDER BY  p.created_at ASC
    LIMIT {}
    "#,
        params, limit
    );

    let mut query = sqlx::query_as::<_, DBTagPhoto>(&query);

    query = query.bind(orientation.to_string());
    for id in ids {
        query = query.bind(id);
    }

    let photos = query.fetch_all(conn).await.context(SqlxSnafu)?;

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
                    orientation: p.orientation,
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

    sqlx::query!(
        r#"
    INSERT INTO photos (id, title, filename, filetype, orientation, created_at, updated_at, deleted)
    VALUES (?, ?, ?, ?, ?, ?, ?, ?)
    "#,
        photo.id,
        photo.title,
        photo.filename,
        photo.filetype,
        photo.orientation,
        photo.created_at,
        photo.updated_at,
        photo.deleted
    )
    .execute(conn)
    .await
    .context(SqlxSnafu)?;

    Ok(id)
}

async fn update(conn: &mut SqliteConnection, photo: DBPhoto) -> Result<(), Error> {
    sqlx::query!(
        r#"
    UPDATE photos
    SET
        title = ?,
        filename = ?,
        filetype = ?,
        orientation = ?,
        updated_at = ?,
        deleted = ?
    WHERE id = ?
    "#,
        photo.title,
        photo.filename,
        photo.filetype,
        photo.orientation,
        photo.updated_at,
        photo.deleted,
        photo.id,
    )
    .execute(conn)
    .await
    .context(SqlxSnafu)?;

    Ok(())
}

async fn attach_tag(conn: &mut SqliteConnection, photo: &Photo, tag: &Tag) -> Result<(), Error> {
    let id = Uuid::new_v4().to_string();

    sqlx::query!(
        r#"
    INSERT INTO photo_tags (id, photo_id, tag_id)
    VALUES (?, ?, ?)
    "#,
        id,
        photo.id,
        tag.id
    )
    .execute(conn)
    .await
    .context(SqlxSnafu)?;

    Ok(())
}

impl TryFrom<DBPhoto> for Photo {
    type Error = Error;

    fn try_from(value: DBPhoto) -> Result<Self, Error> {
        let filetype = FileType::from_str(&value.filetype).context(FileTypeSnafu)?;

        let orientation = Orientation::from_str(&value.orientation).context(OrientationSnafu)?;

        let created_at = {
            let timestamp = value.created_at.0 / 1000;

            OffsetDateTime::from_unix_timestamp(timestamp).context(TimestampSnafu)?
        };

        let updated_at = {
            let timestamp = value.updated_at.0 / 1000;

            OffsetDateTime::from_unix_timestamp(timestamp).context(TimestampSnafu)?
        };

        Ok(Photo {
            id: value.id,
            title: value.title,
            filename: value.filename,
            filetype,
            orientation,
            created_at,
            updated_at,
            deleted: value.deleted,
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
            orientation: photo.orientation.to_string(),
            created_at: photo.created_at.into(),
            updated_at: photo.created_at.into(),
            deleted: photo.deleted,
        }
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to execute query: {}", source))]
    Sqlx { source: SqlxError },

    #[snafu(display("Failed to parse FileType {}", source))]
    FileType {
        source: crate::models::photo::str::filetype::Error,
    },

    #[snafu(display("Failed to parse Orientation {}", source))]
    Orientation { source: strum::ParseError },

    #[snafu(display("Failed to parse timestamp: {}", source))]
    Timestamp { source: time::error::ComponentRange },

    #[snafu(display("Invalid Uuid: {}", source))]
    Uuid { source: UuidError },

    #[snafu(display("Failed to operate on tag: {}", source))]
    Tag { source: TagDbError },
}
