use crate::models::Photo;
use snafu::prelude::*;
use sqlx::error::Error as SqlxError;
use sqlx::SqlitePool;

pub async fn get_all_photos(pool: &SqlitePool) -> Result<Vec<Photo>, Error> {
    let photos = sqlx::query_as!(
        Photo,
        r#"
    SELECT
        id,
        src,
        filename,
        rating,
        filetype AS "filetype: crate::models::FileType",
        date_taken,
        city,
        exif_meta_id,
        created_at,
        updated_at,
        deleted
    FROM
        photos
    WHERE
        deleted = false
    ORDER BY
        created_at DESC
    "#
    )
    .fetch_all(pool)
    .await
    .context(SqlxSnafu)?;

    Ok(photos)
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to execute query: {:?}", source))]
    Sqlx { source: SqlxError },
}
