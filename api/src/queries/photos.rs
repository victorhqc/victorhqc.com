use crate::models::{ExifMeta, FileType, Photo};
use snafu::prelude::*;
use sqlx::error::Error as SqlxError;
use sqlx::SqlitePool;
use std::str::FromStr;
use time::{Date, OffsetDateTime};

#[derive(sqlx::Type, Debug, Clone, sqlx::FromRow)]
#[sqlx(transparent)]
struct Timestamp(i64);

pub async fn get_all_photos(pool: &SqlitePool) -> Result<Vec<Photo>, Error> {
    let rows = sqlx::query!(
        r#"
    SELECT
        p.id as photo_id,
        p.src,
        p.filename,
        p.rating,
        p.filetype,
        p.date_taken as "date_taken: Timestamp",
        p.city,
        p.exif_meta_id,
        p.created_at as "created_at: Timestamp",
        p.updated_at as "updated_at: Timestamp",
        p.deleted
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

    //
    //

    // JOIN exif_metas AS em
    // ON p.exif_meta_id = em.id
    // em.id as meta_id,
    // em.iso,
    // em.focal_length,
    // em.exposure_compensation,
    // em.aperture,
    // em.maker AS "maker: crate::models::Maker",
    // em.crop_factor,
    // em.camera_name,
    // em.lens_name,
    // em.fuji_recipe_id

    let photos: Vec<_> = rows
        .into_iter()
        .map(|row| {
            println!("ROW {:?}", row);

            let filetype = FileType::from_str(&row.filetype)
                .context(FileTypeSnafu)
                .unwrap();

            let date_taken = if let Some(v) = row.date_taken {
                // Time is in milliseconds
                let timestamp = v.0 / 1000;
                let datetime = OffsetDateTime::from_unix_timestamp(timestamp)
                    .context(TimestampSnafu)
                    .unwrap();

                Some(datetime.date())
            } else {
                None
            };

            let created_at =  {
                // Time is in milliseconds
                let timestamp = row.created_at.0 / 1000;
                let datetime = OffsetDateTime::from_unix_timestamp(timestamp)
                    .context(TimestampSnafu)
                    .unwrap();

                datetime
            };

            let updated_at =  {
                // Time is in milliseconds
                let timestamp = row.updated_at.0 / 1000;
                let datetime = OffsetDateTime::from_unix_timestamp(timestamp)
                    .context(TimestampSnafu)
                    .unwrap();

                datetime
            };

            let photo = Photo {
                id: row.photo_id,
                src: row.src,
                filename: row.filename,
                rating: row.rating,
                filetype,
                date_taken,
                city: row.city,
                exif_meta_id: row.exif_meta_id,
                created_at,
                updated_at,
                deleted: row.deleted,
            };

            // let meta = ExifMeta {
            //     id: row.meta_id,
            //     iso: row.iso,
            //     focal_length: row.focal_length,
            //     exposure_compensation: row.exposure_compensation,
            //     aperture: row.aperture,
            //     maker: row.maker,
            //     crop_factor: row.crop_factor,
            //     camera_name: row.camera_name,
            //     lens_name: row.lens_name,
            //     fuji_recipe_id: row.fuji_recipe_id
            // };

            photo
        })
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
