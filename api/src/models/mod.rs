pub mod fujifilm;

use sqlx::FromRow;
use time::OffsetDateTime;
use std::str::FromStr;
use rocket::serde::{Deserialize, Serialize};
use strum_macros::{EnumString, Display as EnumDisplay};
use crate::models::fujifilm::{DRangePriority, DynamicRange, FilmSimulation, GrainSize, GrainStrength, MonochromaticFilter, SettingStrength, TransSensor};


#[derive(Clone, Debug, FromRow)]
pub struct Photo {
    pub id: String,
    pub src: String,
    pub filename: String,
    pub filetype: FileType,
    pub date_taken: OffsetDateTime,
    pub city: String,
    pub exif_meta_id: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted: bool,
}

#[derive(Clone, Debug, FromRow)]
pub struct ExifMeta {
    pub id: String,
    pub iso: i32,
    pub focal_length: f32,
    pub aperture: f32,
    pub maker: Maker,
    pub crop_factor: f32,
    pub camera_name: String,
    pub lens_name: Option<String>,
    fuji_recipe_id: Option<String>,
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

#[derive(Clone, Debug, EnumString, EnumDisplay)]
pub enum FileType {
    JPEG,
}

#[derive(Clone, Debug, EnumString, EnumDisplay)]
pub enum Maker {
    FUJIFILM,
    KONICA,
    CANON,
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
    MonochromeGreen
}