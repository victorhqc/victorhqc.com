mod db;

use rocket::serde::Serialize;
use sqlx::{FromRow, SqlitePool};
use crate::models::Maker;
use snafu::prelude::*;

#[derive(Clone, Debug, Serialize, FromRow)]
pub struct ExifMeta {
    pub id: String,
    pub iso: i64,
    pub focal_length: f64,
    pub exposure_compensation: f64,
    pub aperture: f64,
    pub maker: Maker,
    pub crop_factor: f64,
    pub camera_name: String,
    pub lens_name: Option<String>,
    pub fuji_recipe_id: Option<String>,
}

impl ExifMeta {
    pub async fn find_by_id(pool: &SqlitePool, id: &str) -> Result<ExifMeta, Error> {
        db::find_by_id(pool, id).await.context(DBSnafu)
    }

    pub async fn find_by_ids(pool: &SqlitePool, ids: &Vec<String>) -> Result<Vec<ExifMeta>, Error> {
        db::find_by_ids(pool, ids).await.context(DBSnafu)
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to get photos: {:?}", source))]
    DB { source: db::Error },
}
