pub mod fujifilm;
use crate::models::fujifilm::{
    DRangePriority, DynamicRange, GrainSize, GrainStrength, SettingStrength, TransSensor,
};
use rocket::serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::fmt::Display;
use std::str::FromStr;
// use std::string::ToString;
use strum_macros::{Display as EnumDisplay, EnumString};
use time::{Date, OffsetDateTime};

#[derive(Clone, Debug, FromRow, Serialize)]
pub struct Photo {
    pub id: String,
    pub src: String,
    pub filename: String,
    pub rating: i64,
    pub filetype: FileType,
    pub date_taken: Option<Date>,
    pub city: Option<String>,
    pub exif_meta_id: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted: bool,
}

#[derive(Clone, Debug, FromRow, Serialize)]
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

#[derive(Clone, Debug, FromRow)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted: bool,
}

#[derive(Clone, Debug, FromRow)]
pub struct FujifilmRecipe {
    pub id: String,
    pub author: String,
    pub src: String,
    pub sensor: TransSensor,
    pub film_simulation: FilmSim,
    pub white_balance: WB,
    pub white_balance_shift: String,
    pub dynamic_range: DynamicRange,
    pub d_range_priority: Option<DRangePriority>,
    pub highlight_tone: f32,
    pub shadow_tone: f32,
    pub color: i32,
    pub sharpness: i32,
    pub clarity: Option<i32>,
    pub high_iso_noise_reduction: i32,
    pub grain_strength: Option<GrainStrength>,
    pub grain_size: Option<GrainSize>,
    pub color_chrome_effect: Option<SettingStrength>,
    pub color_chrome_fx_blue: Option<SettingStrength>,
    pub monochromatic_color: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, EnumString, EnumDisplay, sqlx::Type)]
pub enum FileType {
    #[strum(serialize = "JPEG")]
    Jpeg,
}

#[derive(Clone, Debug, Serialize, EnumString, EnumDisplay, sqlx::Type)]
pub enum Maker {
    #[strum(serialize = "FUJIFILM")]
    Fujifilm,
    #[strum(serialize = "KONICA")]
    Konica,
    #[strum(serialize = "CANON")]
    Canon,
}

#[derive(Clone, Debug, EnumString, EnumDisplay)]
pub enum WB {
    Auto,
    AutoWhitePriority,
    AmbiencePriority,
    Custom1,
    Custom2,
    Custom3,
    ColorTemperature,
    Daylight,
    Shade,
    FluorescentLight1,
    FluorescentLight2,
    FluorescentLight3,
    Incandescent,
    Underwater,
}

#[derive(Clone, Debug, EnumString, EnumDisplay)]
pub enum FilmSim {
    ProviaStandard,
    VelviaVivid,
    AstiaSoft,
    ClassicChrome,
    ClassicNeg,
    NostalgicNeg,
    ProNegHi,
    ProNegStd,
    EternaCinema,
    RealaAce,
    Sepia,
    Acros,
    AcrosYellow,
    AcrosRed,
    AcrosGreen,
    Monochrome,
    MonochromeYellow,
    MonochromeRed,
    MonochromeGreen,
}
