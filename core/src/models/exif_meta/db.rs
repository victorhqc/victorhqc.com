use super::{
    str::maker::Error as MakerError, Aperture, CameraMaker, City, DateTaken, ExifMeta,
    ExposureCompensation, FocalLength, Iso, LensMaker, PhotographyDetails, Rating, ShutterSpeed,
};
use snafu::prelude::*;
use sqlx::error::Error as SqlxError;
use sqlx::{FromRow, SqliteConnection};
use std::str::FromStr;
use time::{error::ComponentRange, Date};

#[derive(FromRow)]
struct DBExifMeta {
    pub id: String,
    pub rating: i64,
    pub city: Option<String>,
    pub date_taken: Option<Date>,
    pub iso: i64,
    pub aperture: f64,
    pub shutter_speed: String,
    pub focal_length: f64,
    pub exposure_compensation: f64,
    pub camera_maker: String,
    pub camera_name: String,
    pub lens_maker: String,
    pub lens_name: Option<String>,
    pub crop_factor: f64,
    pub photo_id: String,
    pub fuji_recipe_id: Option<String>,
}

impl ExifMeta {
    pub async fn find_by_id(conn: &mut SqliteConnection, id: &str) -> Result<ExifMeta, Error> {
        find_by_id(conn, id).await
    }

    pub async fn find_by_photo_ids(
        conn: &mut SqliteConnection,
        ids: &Vec<String>,
    ) -> Result<Vec<ExifMeta>, Error> {
        find_by_photo_ids(conn, ids).await
    }

    pub async fn save(&self, conn: &mut SqliteConnection) -> Result<String, Error> {
        let exif: DBExifMeta = self.into();
        insert(conn, exif).await
    }
}

async fn find_by_id(conn: &mut SqliteConnection, id: &str) -> Result<ExifMeta, Error> {
    let exif = sqlx::query_as!(
        DBExifMeta,
        r#"
    SELECT
        id,
        rating,
        city,
        date_taken,
        iso,
        aperture,
        shutter_speed,
        focal_length,
        exposure_compensation,
        camera_maker,
        camera_name,
        lens_maker,
        lens_name,
        crop_factor,
        photo_id,
        fuji_recipe_id
    FROM
        exif_metas
    WHERE
        id = ?
    "#,
        id
    )
    .fetch_one(conn)
    .await
    .context(SqlxSnafu)?;

    exif.try_into()
}

async fn find_by_photo_ids(
    conn: &mut SqliteConnection,
    ids: &Vec<String>,
) -> Result<Vec<ExifMeta>, Error> {
    let params = format!("?{}", ", ?".repeat(ids.len() - 1));

    let query = format!(
        r#"
    SELECT
        id,
        rating,
        city,
        date_taken,
        iso,
        aperture,
        shutter_speed,
        focal_length,
        exposure_compensation,
        camera_maker,
        camera_name,
        lens_maker,
        lens_name,
        crop_factor,
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

    let metas = query.fetch_all(conn).await.context(SqlxSnafu)?;
    let metas: Vec<ExifMeta> = metas.into_iter().map(|m| m.try_into().unwrap()).collect();

    Ok(metas)
}

async fn insert(conn: &mut SqliteConnection, exif: DBExifMeta) -> Result<String, Error> {
    let id = exif.id.clone();

    sqlx::query!(
        r#"
    INSERT INTO exif_metas(
        id,
        rating,
        city,
        date_taken,
        iso,
        aperture,
        shutter_speed,
        focal_length,
        exposure_compensation,
        camera_maker,
        camera_name,
        lens_maker,
        lens_name,
        crop_factor,
        photo_id,
        fuji_recipe_id
    )
    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
    "#,
        exif.id,
        exif.rating,
        exif.city,
        exif.date_taken,
        exif.iso,
        exif.aperture,
        exif.shutter_speed,
        exif.focal_length,
        exif.exposure_compensation,
        exif.camera_maker,
        exif.camera_name,
        exif.lens_maker,
        exif.lens_name,
        exif.crop_factor,
        exif.photo_id,
        exif.fuji_recipe_id,
    )
    .execute(conn)
    .await
    .context(SqlxSnafu)?;

    Ok(id)
}

impl TryFrom<DBExifMeta> for ExifMeta {
    type Error = Error;

    fn try_from(value: DBExifMeta) -> Result<Self, Self::Error> {
        let camera_maker = CameraMaker::from_str(&value.camera_maker).context(CameraMakerSnafu)?;
        let lens_maker = LensMaker::from_str(&value.lens_maker).context(LensMakerSnafu)?;
        let city = value.city.map(City);
        let date_taken = value.date_taken.map(DateTaken);

        Ok(ExifMeta {
            id: value.id,
            photo_id: value.photo_id,
            fuji_recipe_id: value.fuji_recipe_id,
            details: PhotographyDetails {
                rating: Rating(value.rating as i8),
                city,
                date_taken,
                iso: Iso(value.iso),
                aperture: Aperture(value.aperture),
                shutter_speed: ShutterSpeed(value.shutter_speed),
                focal_length: FocalLength {
                    value: value.focal_length,
                    eq_35mm: value.focal_length * value.crop_factor,
                    crop_factor: value.crop_factor,
                },
                exposure_compensation: ExposureCompensation(value.exposure_compensation),
                camera_maker,
                camera_name: value.camera_name,
                lens_maker,
                lens_name: value.lens_name,
            },
        })
    }
}

impl From<&ExifMeta> for DBExifMeta {
    fn from(exif: &ExifMeta) -> Self {
        let city: Option<String> = exif.details.city.clone().map(|c| c.0);
        let date_taken: Option<Date> = exif.details.date_taken.clone().map(|d| d.0);

        DBExifMeta {
            id: exif.id.clone(),
            photo_id: exif.photo_id.clone(),
            fuji_recipe_id: exif.fuji_recipe_id.clone(),
            rating: exif.details.rating.0 as i64,
            date_taken,
            city,
            iso: exif.details.iso.0,
            aperture: exif.details.aperture.0,
            shutter_speed: exif.details.shutter_speed.0.clone(),
            focal_length: exif.details.focal_length.value,
            exposure_compensation: exif.details.exposure_compensation.0,
            camera_maker: exif.details.camera_maker.to_string(),
            camera_name: exif.details.camera_name.clone(),
            lens_maker: exif.details.lens_maker.to_string(),
            lens_name: exif.details.lens_name.clone(),
            crop_factor: exif.details.focal_length.crop_factor,
        }
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to execute query: {:?}", source))]
    Sqlx { source: SqlxError },

    #[snafu(display("Failed to parse Camera Maker {:?}", source))]
    CameraMaker { source: MakerError },

    #[snafu(display("Failed to parse Maker {:?}", source))]
    LensMaker { source: MakerError },

    #[snafu(display("Failed to parse date {:?}", source))]
    Time { source: ComponentRange },
}
