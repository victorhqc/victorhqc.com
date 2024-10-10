pub mod db;
mod from_exif;
pub mod str;

use serde::{Deserialize, Serialize};
use strum_macros::Display as EnumDisplay;

#[derive(Clone, Debug, Serialize)]
pub struct ExifMeta {
    pub id: String,
    pub iso: Iso,
    pub focal_length: FocalLength,
    pub exposure_compensation: ExposureCompensation,
    pub aperture: Aperture,
    pub maker: Maker,
    pub camera_name: String,
    pub lens_name: Option<String>,
    pub photo_id: String,
    pub fuji_recipe_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Iso(pub i64);

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct FocalLength {
    pub value: f64,
    pub eq_35mm: f64,
    pub crop_factor: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ExposureCompensation(pub f64);

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Aperture(pub f64);

#[derive(Clone, Copy, Debug, Deserialize, Serialize, EnumDisplay, sqlx::Type, Eq, PartialEq)]
pub enum Maker {
    #[strum(serialize = "FUJIFILM")]
    Fujifilm,
    #[strum(serialize = "KONICA")]
    Konica,
    #[strum(serialize = "CANON")]
    Canon,
}
