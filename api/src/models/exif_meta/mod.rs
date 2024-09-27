pub mod db;

use crate::models::Maker;
use rocket::serde::Serialize;
use sqlx::FromRow;

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
    pub photo_id: String,
    pub fuji_recipe_id: Option<String>,
}
