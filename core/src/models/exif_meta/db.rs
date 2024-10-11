use super::{
    str::maker::Error as MakerError, Aperture, City, DateTaken, ExifMeta, ExposureCompensation,
    FocalLength, Iso, Maker, PhotographyDetails, Rating,
};
use crate::models::Timestamp;
use snafu::prelude::*;
use sqlx::error::Error as SqlxError;
use sqlx::{FromRow, SqlitePool};
use std::str::FromStr;
use time::{error::ComponentRange, OffsetDateTime};

#[derive(Debug, FromRow)]
struct DBExifMeta {
    pub id: String,
    pub rating: i64,
    pub city: Option<String>,
    pub date_taken: Option<Timestamp>,
    pub iso: i64,
    pub focal_length: f64,
    pub exposure_compensation: f64,
    pub aperture: f64,
    pub maker: String,
    pub crop_factor: f64,
    pub camera_name: String,
    pub lens_name: Option<String>,
    pub photo_id: String,
    pub fuji_recipe_id: Option<String>,
}

impl ExifMeta {
    pub async fn find_by_id(pool: &SqlitePool, id: &str) -> Result<ExifMeta, Error> {
        find_by_id(pool, id).await
    }

    pub async fn find_by_photo_ids(
        pool: &SqlitePool,
        ids: &Vec<String>,
    ) -> Result<Vec<ExifMeta>, Error> {
        find_by_photo_ids(pool, ids).await
    }
}

async fn find_by_id(pool: &SqlitePool, id: &str) -> Result<ExifMeta, Error> {
    let exif = sqlx::query_as!(
        DBExifMeta,
        r#"
    SELECT
        id,
        rating,
        city,
        date_taken as "date_taken: Timestamp",
        iso,
        focal_length,
        exposure_compensation,
        aperture,
        maker,
        crop_factor,
        camera_name,
        lens_name,
        photo_id,
        fuji_recipe_id
    FROM
        exif_metas
    WHERE
        id = ?
    "#,
        id
    )
    .fetch_one(pool)
    .await
    .context(SqlxSnafu)?;

    exif.try_into()
}

async fn find_by_photo_ids(pool: &SqlitePool, ids: &Vec<String>) -> Result<Vec<ExifMeta>, Error> {
    let params = format!("?{}", ", ?".repeat(ids.len() - 1));

    let query = format!(
        r#"
    SELECT
        id,
        rating,
        city,
        date_taken as "date_taken: Timestamp",
        iso,
        focal_length,
        exposure_compensation,
        aperture,
        maker,
        crop_factor,
        camera_name,
        lens_name,
        photo_id,
        fuji_recipe_id
    FROM
        exif_metas
    WHERE
        photo_id IN ( { } )
    "#,
        params
    );

    let mut query = sqlx::query_as::<_, DBExifMeta>(&query);

    for id in ids {
        query = query.bind(id);
    }

    let metas = query.fetch_all(pool).await.context(SqlxSnafu)?;
    let metas: Vec<ExifMeta> = metas.into_iter().map(|m| m.try_into().unwrap()).collect();

    Ok(metas)
}

impl TryFrom<DBExifMeta> for ExifMeta {
    type Error = Error;

    fn try_from(value: DBExifMeta) -> Result<Self, Self::Error> {
        let maker = Maker::from_str(&value.maker).context(MakerSnafu)?;
        let city = value.city.map(City);
        let date_taken = match value.date_taken {
            Some(d) => {
                let d = d.0 / 1000;
                let date = match OffsetDateTime::from_unix_timestamp(d) {
                    Ok(d) => d,
                    Err(err) => return Err(Error::Time { source: err }),
                };

                Some(DateTaken(date))
            }
            None => None,
        };

        Ok(ExifMeta {
            id: value.id,
            photo_id: value.photo_id,
            fuji_recipe_id: value.fuji_recipe_id,
            details: PhotographyDetails {
                rating: Rating(value.rating as i8),
                city,
                date_taken,
                iso: Iso(value.iso),
                focal_length: FocalLength {
                    value: value.focal_length,
                    eq_35mm: value.focal_length * value.crop_factor,
                    crop_factor: value.crop_factor,
                },
                exposure_compensation: ExposureCompensation(value.exposure_compensation),
                aperture: Aperture(value.aperture),
                maker,
                camera_name: value.camera_name,
                lens_name: value.lens_name,
            },
        })
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to execute query: {:?}", source))]
    Sqlx { source: SqlxError },

    #[snafu(display("Failed to parse Maker {:?}", source))]
    Maker { source: MakerError },

    #[snafu(display("Failed to parse date {:?}", source))]
    Time { source: ComponentRange },
}
