pub mod exif_meta;
pub mod fujifilm;
pub mod photo;
pub mod tag;

use crate::models::fujifilm::{
    DRangePriority, DynamicRange, GrainSize, GrainStrength, SettingStrength, TransSensor,
};
use rocket::serde::{Deserialize, Serialize};
use strum_macros::{Display as EnumDisplay, EnumString};

#[derive(sqlx::Type, Debug, Clone)]
#[sqlx(transparent)]
pub struct Timestamp(i64);

#[derive(Clone, Debug)]
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
