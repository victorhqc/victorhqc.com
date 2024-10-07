use crate::exif::{ExifData, FindExifData, FromExifData};
use crate::models::fujifilm::{ColorChromeEffect, SettingStrength};
use log::debug;

impl FromExifData for ColorChromeEffect {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        let exif = data.find("ColorChromeEffect")?;

        debug!("ColorChromeEffect::from_exif: \"{}\"", exif);

        match exif.value().to_lowercase().as_str() {
            "strong" => Some(ColorChromeEffect {
                strength: SettingStrength::Strong,
            }),
            "weak" => Some(ColorChromeEffect {
                strength: SettingStrength::Weak,
            }),
            _ => Some(ColorChromeEffect {
                strength: SettingStrength::Off,
            }),
        }
    }
}
