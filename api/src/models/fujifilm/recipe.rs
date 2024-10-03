use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
use strum_macros::{Display, EnumString};

#[derive(Debug, Serialize, Deserialize)]
pub struct FujifilmRecipe {
    pub name: String,
    pub src: String,
    pub film_simulation: FilmSimulation,
    pub sensor: TransSensor,
    pub settings: Settings,
}

#[derive(Debug, Serialize, Deserialize, Display)]
#[serde(untagged)]
pub enum Settings {
    TransI(TransISettings),
    TransII(TransIISettings),
    TransIII(TransIIISettings),
    TransIV(TransIVSettings),
    TransV(TransVSettings),
}

#[derive(Debug, Deserialize, PartialEq, Display, EnumString, Clone)]
pub enum TransSensor {
    #[strum(serialize = "Trans Sensor I", to_string = "TransI")]
    TransI,
    #[strum(serialize = "Trans Sensor II", to_string = "TransII")]
    TransII,
    #[strum(serialize = "Trans Sensor III", to_string = "TransIII")]
    TransIII,
    #[strum(serialize = "Trans Sensor IV", to_string = "TransIV")]
    TransIV,
    #[strum(serialize = "Trans Sensor V", to_string = "TransV")]
    TransV,
}

impl Serialize for TransSensor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Debug, Deserialize, PartialEq, Display)]
pub enum FilmSimulation {
    #[strum(serialize = "Provia", to_string = "Provia")]
    ProviaStandard,
    #[strum(serialize = "Velvia", to_string = "Velvia")]
    VelviaVivid,
    #[strum(serialize = "Astia", to_string = "Astia")]
    AstiaSoft,
    #[strum(serialize = "Classic Chrome", to_string = "Classic Chrome")]
    ClassicChrome,
    #[strum(serialize = "Reala Ace", to_string = "Reala Ace")]
    RealaAce,
    #[strum(serialize = "Pro Neg. Hi", to_string = "Pro Neg. Hi")]
    ProNegHi,
    #[strum(serialize = "Pro Neg. Std", to_string = "Pro Neg. Std")]
    ProNegStd,
    #[strum(serialize = "Classic Negative", to_string = "Classic Negative")]
    ClassicNeg,
    #[strum(serialize = "Nostalgic Negative", to_string = "Nostalgic Negative")]
    NostalgicNeg,
    #[strum(serialize = "Eterna", to_string = "Eterna")]
    EternaCinema,
    #[strum(serialize = "Eterna Bleach Bypass", to_string = "Eterna Bleach Bypass")]
    BleachBypass,
    #[strum(serialize = "Acros{filter}", to_string = "Acros{filter}")]
    Acros { filter: MonochromaticFilter },
    #[strum(serialize = "Monochrome{filter}")]
    Monochrome { filter: MonochromaticFilter },
    #[strum(serialize = "Sepia", to_string = "Sepia")]
    Sepia,
}

impl Serialize for FilmSimulation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Display, EnumString, Default)]
pub enum MonochromaticFilter {
    #[strum(serialize = "Standard", to_string = "")]
    #[default]
    Std,
    #[strum(serialize = "Yellow", to_string = " +Ye")]
    Yellow,
    #[strum(serialize = "Red", to_string = " +R")]
    Red,
    #[strum(serialize = "Green", to_string = " +G")]
    Green,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Display, EnumString, Default)]
pub enum GrainStrength {
    #[default]
    #[strum(to_string = "Weak")]
    Weak,
    #[strum(to_string = "Strong")]
    Strong,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Display, EnumString, Default)]
pub enum GrainSize {
    #[default]
    #[strum(to_string = "Small")]
    Small,
    #[strum(to_string = "Large")]
    Large,
}

#[derive(Debug, Clone, Deserialize, Display, PartialEq, Default)]
pub enum GrainEffect {
    #[default]
    #[strum(to_string = "Off")]
    Off,
    #[strum(to_string = "{strength}")]
    OnlyStrength { strength: GrainStrength },
    #[strum(to_string = "{strength}, {size}")]
    StrengthAndSize {
        strength: GrainStrength,
        size: GrainSize,
    },
}

impl Serialize for GrainEffect {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Display, EnumString, Default)]
pub enum SettingStrength {
    #[default]
    #[strum(serialize = "Off")]
    Off,
    #[strum(serialize = "Weak")]
    Weak,
    #[strum(serialize = "Strong")]
    Strong,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum WhiteBalance {
    Auto { shift: WBShift },
    AutoWhitePriority { shift: WBShift },
    AutoAmbiencePriority { shift: WBShift },
    Custom1 { shift: WBShift },
    Custom2 { shift: WBShift },
    Custom3 { shift: WBShift },
    Kelvin { temperature: i32, shift: WBShift },
    Daylight { shift: WBShift },
    Cloudy { shift: WBShift },
    FluorescentLight1 { shift: WBShift },
    FluorescentLight2 { shift: WBShift },
    FluorescentLight3 { shift: WBShift },
    Incandescent { shift: WBShift },
    Underwater { shift: WBShift },
}

impl WhiteBalance {
    pub fn set_shift(&mut self, s: WBShift) {
        match self {
            WhiteBalance::Auto { shift }
            | WhiteBalance::AutoWhitePriority { shift }
            | WhiteBalance::AutoAmbiencePriority { shift }
            | WhiteBalance::Custom1 { shift }
            | WhiteBalance::Custom2 { shift }
            | WhiteBalance::Custom3 { shift }
            | WhiteBalance::Daylight { shift }
            | WhiteBalance::Cloudy { shift }
            | WhiteBalance::FluorescentLight1 { shift }
            | WhiteBalance::FluorescentLight2 { shift }
            | WhiteBalance::FluorescentLight3 { shift }
            | WhiteBalance::Incandescent { shift }
            | WhiteBalance::Underwater { shift } => *shift = s,
            WhiteBalance::Kelvin { shift, .. } => {
                *shift = WBShift {
                    blue: s.blue,
                    red: s.red,
                }
            }
        }
    }
}

impl Default for WhiteBalance {
    fn default() -> Self {
        WhiteBalance::Auto {
            shift: WBShift::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct WBShift {
    pub red: i32,
    pub blue: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Display, EnumString, Default)]
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Display, EnumString, Default)]
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

#[derive(Debug, Serialize, PartialEq, Deserialize)]
#[serde(tag = "type")]
pub enum MonochromaticColor {
    ColorShift { shift: MonochromaticColorShift },
    Strength { value: i32 },
}

impl Default for MonochromaticColor {
    fn default() -> Self {
        MonochromaticColor::ColorShift {
            shift: MonochromaticColorShift { mg: 0, wc: 0 },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct MonochromaticColorShift {
    pub wc: i32,
    pub mg: i32,
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
pub struct ToneCurve {
    pub highlights: f64,
    pub shadows: f64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Color {
    pub value: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Sharpness {
    pub value: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HighISONoiseReduction {
    pub value: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Clarity {
    pub value: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransVSettings {
    pub white_balance: WhiteBalance,
    pub dynamic_range: DynamicRange,
    pub d_range_priority: DRangePriority,
    pub grain_effect: GrainEffect,
    pub color_chrome_effect: ColorChromeEffect,
    pub color_chrome_fx_blue: ColorChromeEffectFxBlue,
    pub tone_curve: ToneCurve,
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
    pub tone_curve: ToneCurve,
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
    pub tone_curve: ToneCurve,
    pub color: Color,
    pub monochromatic_color: MonochromaticColor,
    pub sharpness: Sharpness,
    pub high_iso_noise_reduction: HighISONoiseReduction,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransIISettings {
    pub white_balance: WhiteBalance,
    pub dynamic_range: DynamicRange,
    pub tone_curve: ToneCurve,
    pub color: Color,
    pub sharpness: Sharpness,
    pub high_iso_noise_reduction: HighISONoiseReduction,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransISettings {
    pub white_balance: WhiteBalance,
    pub dynamic_range: DynamicRange,
    pub tone_curve: ToneCurve,
    pub color: Color,
    pub sharpness: Sharpness,
    pub high_iso_noise_reduction: HighISONoiseReduction,
}
