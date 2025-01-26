use super::Tag;
use crate::models::Timestamp;
use snafu::prelude::*;
use sqlx::{Error as SqlxError, FromRow, SqliteConnection};
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
    pub async fn find_by_name(conn: &mut SqliteConnection, name: &str) -> Result<Tag, Error> {
        let tags = find_by_names(conn, &[name]).await?;

        let tag = tags.first().context(TagsNotFoundSnafu {
            name: name.to_string(),
        })?;

        Ok(tag.clone())
    }

    pub async fn find_by_names(
        conn: &mut SqliteConnection,
        names: &[&str],
    ) -> Result<Vec<Tag>, Error> {
        let tags = find_by_names(conn, names).await?;

        Ok(tags)
    }

    pub async fn find_by_name_or_create(
        conn: &mut SqliteConnection,
        name: &str,
    ) -> Result<Tag, Error> {
        let tag = Tag::find_by_name(conn, name).await;

        match tag {
            Ok(tag) => Ok(tag),
            Err(e) => match e {
                Error::TagsNotFound { .. } => {
                    let tag = Tag::new(name.to_string());
                    tag.save(conn).await?;

                    Ok(tag)
                }
                _ => Err(e),
            },
        }
    }

    pub async fn find_by_ids(
        conn: &mut SqliteConnection,
        ids: &Vec<String>,
    ) -> Result<Vec<Tag>, Error> {
        find_by_ids(conn, ids).await
    }

    pub async fn find_by_photo_ids(
        conn: &mut SqliteConnection,
        ids: &Vec<String>,
    ) -> Result<Vec<(String, Tag)>, Error> {
        find_by_photo_ids(conn, ids).await
    }

    pub async fn save(&self, conn: &mut SqliteConnection) -> Result<String, Error> {
        let tag: DBTag = self.into();
        insert(conn, &tag).await
    }
}

async fn find_by_names(conn: &mut SqliteConnection, names: &[&str]) -> Result<Vec<Tag>, Error> {
    let params = format!("?{}", ", ?".repeat(names.len() - 1));

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
        AND name IN ({ })
    "#,
        params
    );

    let mut query = sqlx::query_as::<_, DBTag>(&query);

    for name in names {
        query = query.bind(name);
    }

    let tags = query.fetch_all(conn).await.context(SqlxSnafu)?;

    let tags: Vec<Tag> = tags.into_iter().map(|t| t.try_into().unwrap()).collect();

    Ok(tags)
}

async fn find_by_ids(conn: &mut SqliteConnection, ids: &Vec<String>) -> Result<Vec<Tag>, Error> {
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

    let tags = query.fetch_all(conn).await.context(SqlxSnafu)?;

    let tags: Vec<Tag> = tags.into_iter().map(|t| t.try_into().unwrap()).collect();

    Ok(tags)
}

async fn find_by_photo_ids(
    conn: &mut SqliteConnection,
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

    let tags = query.fetch_all(conn).await.context(SqlxSnafu)?;

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

async fn insert(conn: &mut SqliteConnection, tag: &DBTag) -> Result<String, Error> {
    let id = tag.id.clone();

    sqlx::query!(
        r#"
    INSERT INTO tags(id, name, created_at, updated_at, deleted)
    VALUES (?, ?, ?, ?, ?)
    "#,
        tag.id,
        tag.name,
        tag.created_at,
        tag.updated_at,
        tag.deleted
    )
    .execute(conn)
    .await
    .context(SqlxSnafu)?;

    Ok(id)
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

impl From<&Tag> for DBTag {
    fn from(value: &Tag) -> Self {
        DBTag {
            id: value.id.clone(),
            name: value.name.clone(),
            created_at: value.created_at.into(),
            updated_at: value.updated_at.into(),
            deleted: value.deleted,
        }
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to execute query: {:?}", source))]
    Sqlx { source: SqlxError },

    #[snafu(display("Failed to parse timestamp: {:?}", source))]
    Timestamp { source: time::error::ComponentRange },

    #[snafu(display("Could not find tags for: {}", name))]
    TagsNotFound { name: String },
}
