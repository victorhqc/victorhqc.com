use crate::exif::{ExifData, FindExifData, FromExifData};
use crate::models::fujifilm::{ColorChromeEffectFxBlue, SettingStrength};

impl FromExifData for ColorChromeEffectFxBlue {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        let exif = data.find("ColorChromeFXBlue")?;

        match exif.value().to_lowercase().as_str() {
            "strong" => Some(ColorChromeEffectFxBlue {
                strength: SettingStrength::Strong,
            }),
            "weak" => Some(ColorChromeEffectFxBlue {
                strength: SettingStrength::Weak,
            }),
            _ => Some(ColorChromeEffectFxBlue {
                strength: SettingStrength::Off,
            }),
        }
    }
}
