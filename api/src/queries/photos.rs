use crate::models::{ExifMeta, Photo};
use snafu::prelude::*;
use sqlx::error::Error as SqlxError;
use sqlx::SqlitePool;

pub async fn get_all_photos(pool: &SqlitePool) -> Result<Vec<(Photo, ExifMeta)>, Error> {
    let rows = sqlx::query!(
        r#"
    SELECT
        p.id as photo_id,
        p.src,
        p.filename,
        p.rating,
        p.filetype AS "filetype: crate::models::FileType",
        p.date_taken,
        p.city,
        p.exif_meta_id,
        p.created_at,
        p.updated_at,
        p.deleted,
        em.id as meta_id,
        em.iso,
        em.focal_length,
        em.exposure_compensation,
        em.aperture,
        em.maker AS "maker: crate::models::Maker",
        em.crop_factor,
        em.camera_name,
        em.lens_name,
        em.fuji_recipe_id
    FROM
        photos AS p
    JOIN exif_metas AS em
        ON p.exif_meta_id = em.id
    WHERE
        deleted = false
    ORDER BY
        created_at DESC
    "#
    )
    .fetch_all(pool)
    .await
    .context(SqlxSnafu)?;

    let photos: Vec<_> = rows.into_iter().map(|row| {
        let photo = Photo {
            id: row.photo_id,
            src: row.src,
            filename: row.filename,
            rating: row.rating,
            filetype: row.filetype,
            date_taken: row.date_taken,
            city: row.city,
            exif_meta_id: row.exif_meta_id,
            created_at: row.created_at,
            updated_at: row.updated_at,
            deleted: row.deleted,
        };

        let meta = ExifMeta {
            id: row.meta_id,
            iso: row.iso,
            focal_length: row.focal_length,
            exposure_compensation: row.exposure_compensation,
            aperture: row.aperture,
            maker: row.maker,
            crop_factor: row.crop_factor,
            camera_name: row.camera_name,
            lens_name: row.lens_name,
            fuji_recipe_id: row.fuji_recipe_id
        };

        (photo, meta)
    }).collect();

    Ok(photos)
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to execute query: {:?}", source))]
    Sqlx { source: SqlxError },
}
