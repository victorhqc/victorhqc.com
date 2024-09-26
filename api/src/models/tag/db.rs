use super::Tag;
use crate::models::Timestamp;
use snafu::prelude::*;
use sqlx::{Error as SqlxError, FromRow, SqlitePool};
use time::OffsetDateTime;

#[derive(FromRow)]
struct DBTag {
    pub id: String,
    pub name: String,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub deleted: bool,
}

#[derive(FromRow, Debug)]
struct DBPhotoTag {
    pub photo_id: String,
    pub id: String,
    pub name: String,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub deleted: bool,
}

impl Tag {
    pub async fn find_by_name(pool: &SqlitePool, name: &str) -> Result<Tag, Error> {
        find_by_name(pool, name).await
    }

    pub async fn find_by_ids(pool: &SqlitePool, ids: &Vec<String>) -> Result<Vec<Tag>, Error> {
        find_by_ids(pool, ids).await
    }

    pub async fn find_by_photo_ids(
        pool: &SqlitePool,
        ids: &Vec<String>,
    ) -> Result<Vec<(String, Tag)>, Error> {
        find_by_photo_ids(pool, ids).await
    }
}

async fn find_by_name(pool: &SqlitePool, id: &str) -> Result<Tag, Error> {
    let tag = sqlx::query_as!(
        DBTag,
        r#"
    SELECT
        id,
        name,
        created_at as "created_at: Timestamp",
        updated_at as "updated_at: Timestamp",
        deleted
    FROM
        tags
    WHERE
        deleted = false
        AND name = ?
    "#,
        id
    )
    .fetch_one(pool)
    .await
    .context(SqlxSnafu)?;

    Ok(tag.try_into()?)
}

async fn find_by_ids(pool: &SqlitePool, ids: &Vec<String>) -> Result<Vec<Tag>, Error> {
    let params = format!("?{}", ", ?".repeat(ids.len() - 1));

    let query = format!(
        r#"
    SELECT
        id,
        name,
        created_at,
        updated_at,
        deleted
    FROM
        tags
    WHERE
        deleted = false
        AND id IN ( { } )
    "#,
        params
    );

    let mut query = sqlx::query_as::<_, DBTag>(&query);

    for id in ids {
        query = query.bind(id);
    }

    let tags = query.fetch_all(pool).await.context(SqlxSnafu)?;

    let tags: Vec<Tag> = tags.into_iter().map(|t| t.try_into().unwrap()).collect();

    Ok(tags)
}

async fn find_by_photo_ids(
    pool: &SqlitePool,
    ids: &Vec<String>,
) -> Result<Vec<(String, Tag)>, Error> {
    let params = format!("?{}", ", ?".repeat(ids.len() - 1));

    let query = format!(
        r#"
    SELECT
        pt.photo_id,
        t.id,
        t.name,
        t.created_at,
        t.updated_at,
        deleted
    FROM
        tags as t
    JOIN
        photo_tags as pt ON pt.tag_id = t.id
    WHERE
        pt.photo_id IN ( { } )
        AND deleted = false
    "#,
        params
    );

    let mut query = sqlx::query_as::<_, DBPhotoTag>(&query);

    for id in ids {
        query = query.bind(id);
    }

    let tags = query.fetch_all(pool).await.context(SqlxSnafu)?;

    let tags: Vec<(String, Tag)> = tags
        .into_iter()
        .map(|t| {
            (
                t.photo_id,
                DBTag {
                    id: t.id,
                    name: t.name,
                    created_at: t.created_at,
                    updated_at: t.updated_at,
                    deleted: t.deleted,
                },
            )
        })
        .map(|(id, t)| (id, t.try_into().unwrap()))
        .collect();

    Ok(tags)
}

impl TryFrom<DBTag> for Tag {
    type Error = Error;
    fn try_from(value: DBTag) -> Result<Self, Self::Error> {
        let created_at = {
            let timestamp = value.created_at.0 / 1000;

            OffsetDateTime::from_unix_timestamp(timestamp).context(TimestampSnafu)?
        };

        let updated_at = {
            let timestamp = value.updated_at.0 / 1000;

            OffsetDateTime::from_unix_timestamp(timestamp).context(TimestampSnafu)?
        };

        Ok(Tag {
            id: value.id,
            name: value.name,
            created_at,
            updated_at,
            deleted: value.deleted,
        })
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to execute query: {:?}", source))]
    Sqlx { source: SqlxError },

    #[snafu(display("Failed to parse timestamp: {:?}", source))]
    Timestamp { source: time::error::ComponentRange },
}
