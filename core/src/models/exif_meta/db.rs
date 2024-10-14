use super::{
    str::maker::Error as MakerError, Aperture, City, DateTaken, ExifMeta, ExposureCompensation,
    FocalLength, Iso, Maker, PhotographyDetails, Rating,
};
use crate::models::Timestamp;
use snafu::prelude::*;
use sqlx::error::Error as SqlxError;
use sqlx::{FromRow, SqliteConnection, SqlitePool};
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

    pub async fn save(&self, pool: &mut SqliteConnection) -> Result<String, Error> {
        let exif: DBExifMeta = self.into();
        insert(pool, exif).await
    }
}

async fn find_by_id(pool: &SqlitePool, id: &str) -> Result<ExifMeta, Error> {
    // TODO: Move back to macro. Fails to compile in IDE because fails to find DB
    let exif = sqlx::query_as::<_, DBExifMeta>(
        r#"
    SELECT
        id,
        rating,
        city,
        date_taken,
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
    )
    .bind(id)
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
        date_taken,
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

async fn insert(conn: &mut SqliteConnection, exif: DBExifMeta) -> Result<String, Error> {
    let id = exif.id.clone();

    sqlx::query(
        r#"
    INSERT INTO exif_metas(
        id,
        rating,
        city,
        date_taken,
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
    )
    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
    "#,
    )
    .bind(exif.id)
    .bind(exif.rating)
    .bind(exif.city)
    .bind(exif.date_taken)
    .bind(exif.iso)
    .bind(exif.focal_length)
    .bind(exif.exposure_compensation)
    .bind(exif.aperture)
    .bind(exif.maker)
    .bind(exif.crop_factor)
    .bind(exif.camera_name)
    .bind(exif.lens_name)
    .bind(exif.photo_id)
    .bind(exif.fuji_recipe_id)
    .execute(conn)
    .await
    .context(SqlxSnafu)?;

    Ok(id)
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

impl From<&ExifMeta> for DBExifMeta {
    fn from(exif: &ExifMeta) -> Self {
        let city: Option<String> = exif.details.city.clone().map(|c| c.0);
        let date_taken: Option<Timestamp> = exif.details.date_taken.clone().map(|d| d.0.into());

        DBExifMeta {
            id: exif.id.clone(),
            photo_id: exif.photo_id.clone(),
            fuji_recipe_id: exif.fuji_recipe_id.clone(),
            rating: exif.details.rating.0 as i64,
            date_taken,
            city,
            iso: exif.details.iso.0,
            focal_length: exif.details.focal_length.value,
            exposure_compensation: exif.details.exposure_compensation.0,
            aperture: exif.details.aperture.0,
            maker: exif.details.maker.to_string(),
            crop_factor: exif.details.focal_length.crop_factor,
            camera_name: exif.details.camera_name.clone(),
            lens_name: exif.details.lens_name.clone(),
        }
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
