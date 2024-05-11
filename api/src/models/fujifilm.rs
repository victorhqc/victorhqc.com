use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Debug, Serialize, Deserialize)]
pub struct FujifilmRecipe {
    pub meta: Meta,
    pub film_simulation: FilmSimulation,
    pub sensor: TransSensor,
    pub settings: Settings,
}

#[derive(Debug, Serialize, Deserialize, Display)]
pub enum Settings {
    TransV(TransVSettings),
    TransIV(TransIVSettings),
    TransIII(TransIIISettings),
    TransII(TransIISettings),
    TransI(TransISettings),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Display, Clone)]
pub enum TransSensor {
    #[strum(serialize = "Trans Sensor I")]
    TransI,
    #[strum(serialize = "Trans Sensor II")]
    TransII,
    #[strum(serialize = "Trans Sensor III")]
    TransIII,
    #[strum(serialize = "Trans Sensor IV")]
    TransIV,
    #[strum(serialize = "Trans Sensor V")]
    TransV,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub name: String,
    pub author: Option<String>,
    pub src: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Display)]
pub enum FilmSimulation {
    #[strum(serialize = "Provia")]
    ProviaStandard,
    #[strum(serialize = "Velvia")]
    VelviaVivid,
    #[strum(serialize = "Astia")]
    AstiaSoft,
    #[strum(serialize = "Classic Chrome")]
    ClassicChrome,
    #[strum(serialize = "Classic Neg.")]
    ClassicNeg,
    #[strum(serialize = "Nostalgic Neg.")]
    NostalgicNeg,
    #[strum(serialize = "Pro Neg. Hi")]
    ProNegHi,
    #[strum(serialize = "Pro Neg. Std")]
    ProNegStd,
    #[strum(serialize = "Acros")]
    Acros(Option<MonochromaticFilter>),
    #[strum(serialize = "Eterna")]
    EternaCinema,
    #[strum(serialize = "Monochrome")]
    Monochrome(Option<MonochromaticFilter>),
    #[strum(serialize = "Reala")]
    RealaAce,
    #[strum(serialize = "Sepia")]
    Sepia,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Display, EnumString)]
pub enum MonochromaticFilter {
    #[strum(serialize = "None")]
    None,
    #[strum(serialize = "Green")]
    Green,
    #[strum(serialize = "Red")]
    Red,
    #[strum(serialize = "Blue")]
    Blue,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Display, EnumString, Default)]
pub enum GrainStrength {
    #[strum(serialize = "Weak")]
    #[default]
    Weak,
    #[strum(serialize = "Strong")]
    Strong,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Display, EnumString, Default)]
pub enum GrainSize {
    #[strum(serialize = "Small")]
    #[default]
    Small,
    #[strum(serialize = "Large")]
    Large,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub enum GrainEffect {
    #[default]
    Off,
    OnlyStrength(GrainStrength),
    StrengthAndSize(GrainStrength, GrainSize),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Display, EnumString, Default)]
pub enum SettingStrength {
    #[default]
    #[strum(serialize = "Off")]
    Off,
    #[strum(serialize = "Weak")]
    Weak,
    #[strum(serialize = "Strong")]
    Strong,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum WhiteBalance {
    Auto(WBShift),
    AutoWhitePriority(WBShift),
    AmbiencePriority(WBShift),
    Custom1(WBShift),
    Custom2(WBShift),
    Custom3(WBShift),
    ColorTemperature(WBShift, i32),
    Daylight(WBShift),
    Shade(WBShift),
    FluorescentLight1(WBShift),
    FluorescentLight2(WBShift),
    FluorescentLight3(WBShift),
    Incandescent(WBShift),
    Underwater(WBShift),
}

impl Default for WhiteBalance {
    fn default() -> Self {
        WhiteBalance::Auto(WBShift::default())
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WBShift {
    pub red: i32,
    pub blue: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Display, Default)]
pub enum DynamicRange {
    #[default]
    #[strum(serialize = "Auto")]
    Auto,
    #[strum(serialize = "100%")]
    DR100,
    #[strum(serialize = "200%")]
    DR200,
    #[strum(serialize = "400%")]
    DR400,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Display, Default)]
pub enum DRangePriority {
    #[default]
    #[strum(serialize = "Off")]
    Off,
    #[strum(serialize = "Auto")]
    Auto,
    #[strum(serialize = "Weak")]
    Weak,
    #[strum(serialize = "Strong")]
    Strong,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MonochromaticColor {
    ColorShift(MonochromaticColorShift),
    Strength(MonochromaticEffect),
}

impl Default for MonochromaticColor {
    fn default() -> Self {
        MonochromaticColor::ColorShift(MonochromaticColorShift { mg: 0, wc: 0 })
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MonochromaticColorShift {
    pub wc: i32,
    pub mg: i32,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MonochromaticEffect {
    pub strength: i32,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ColorChromeEffect {
    pub strength: SettingStrength,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ColorChromeEffectFxBlue {
    pub strength: SettingStrength,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HighlightTone {
    pub value: f32,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ShadowTone {
    pub value: f32,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Color {
    pub value: i32,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Sharpness {
    pub value: i32,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HighISONoiseReduction {
    pub value: i32,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Clarity {
    pub value: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransVSettings {
    pub white_balance: WhiteBalance,
    pub dynamic_range: DynamicRange,
    pub d_range_priority: DRangePriority,
    pub grain_effect: GrainEffect,
    pub color_chrome_effect: ColorChromeEffect,
    pub color_chrome_fx_blue: ColorChromeEffectFxBlue,
    pub highlight_tone: HighlightTone,
    pub shadow_tone: ShadowTone,
    pub color: Color,
    pub monochromatic_color: MonochromaticColor,
    pub sharpness: Sharpness,
    pub high_iso_noise_reduction: HighISONoiseReduction,
    pub clarity: Clarity,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransIVSettings {
    pub white_balance: WhiteBalance,
    pub dynamic_range: DynamicRange,
    pub d_range_priority: DRangePriority,
    pub grain_effect: GrainEffect,
    pub color_chrome_fx_blue: ColorChromeEffectFxBlue,
    pub color_chrome_effect: ColorChromeEffect,
    pub highlight_tone: HighlightTone,
    pub shadow_tone: ShadowTone,
    pub color: Color,
    pub monochromatic_color: MonochromaticColor,
    pub sharpness: Sharpness,
    pub high_iso_noise_reduction: HighISONoiseReduction,
    pub clarity: Clarity,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransIIISettings {
    pub white_balance: WhiteBalance,
    pub dynamic_range: DynamicRange,
    pub grain_effect: GrainEffect,
    pub highlight_tone: HighlightTone,
    pub shadow_tone: ShadowTone,
    pub color: Color,
    pub monochromatic_color: MonochromaticColor,
    pub sharpness: Sharpness,
    pub high_iso_noise_reduction: HighISONoiseReduction,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransIISettings {
    pub white_balance: WhiteBalance,
    pub dynamic_range: DynamicRange,
    pub highlight_tone: HighlightTone,
    pub shadow_tone: ShadowTone,
    pub color: Color,
    pub sharpness: Sharpness,
    pub high_iso_noise_reduction: HighISONoiseReduction,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransISettings {
    pub white_balance: WhiteBalance,
    pub dynamic_range: DynamicRange,
    pub highlight_tone: HighlightTone,
    pub shadow_tone: ShadowTone,
    pub color: Color,
    pub sharpness: Sharpness,
    pub high_iso_noise_reduction: HighISONoiseReduction,
}
