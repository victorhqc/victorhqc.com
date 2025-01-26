pub mod db;
mod from_exif;
pub mod str;

use crate::models::fujifilm::FujifilmRecipe;
use crate::models::photo::Photo;
use serde::{Deserialize, Serialize};
use strum_macros::Display as EnumDisplay;
use time::Date;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExifMeta {
    pub id: String,
    pub photo_id: String,
    pub fuji_recipe_id: Option<String>,
    pub details: PhotographyDetails,
}

impl ExifMeta {
    pub fn new(
        details: PhotographyDetails,
        photo: &Photo,
        recipe: &Option<FujifilmRecipe>,
    ) -> Self {
        let id = Uuid::new_v4().to_string();
        let fuji_recipe_id: Option<String> = recipe.clone().map(|r| r.id);

        ExifMeta {
            id,
            photo_id: photo.id.clone(),
            fuji_recipe_id,
            details,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PhotographyDetails {
    pub rating: Rating,
    pub date_taken: Option<DateTaken>,
    pub city: Option<City>,
    pub iso: Iso,
    pub aperture: Aperture,
    pub shutter_speed: ShutterSpeed,
    pub focal_length: FocalLength,
    pub exposure_compensation: ExposureCompensation,
    pub camera_maker: CameraMaker,
    pub lens_maker: LensMaker,
    pub camera_name: String,
    pub lens_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Rating(pub i8);

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DateTaken(pub Date);

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct City(pub String);

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

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ShutterSpeed(pub String);

#[derive(Clone, Copy, Debug, Deserialize, Serialize, EnumDisplay, sqlx::Type, Eq, PartialEq)]
pub enum CameraMaker {
    #[strum(serialize = "FUJIFILM")]
    Fujifilm,
    #[strum(serialize = "KONICA")]
    Konica,
    #[strum(serialize = "CANON")]
    Canon,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, EnumDisplay, sqlx::Type, Eq, PartialEq)]
pub enum LensMaker {
    #[strum(serialize = "FUJIFILM")]
    Fujifilm,
    #[strum(serialize = "KONICA")]
    Konica,
    #[strum(serialize = "CANON")]
    Canon,
    #[strum(serialize = "7Artisans")]
    SevenArtisans,
    #[strum(serialize = "Unknown")]
    Unknown,
}
