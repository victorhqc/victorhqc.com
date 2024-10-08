use crate::exif::{ExifData, FindExifData, FromExifData};
use crate::models::fujifilm::Color;
use log::debug;

impl FromExifData for Color {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        let exif = data.find("Saturation")?;

        debug!("Color::from_exif: {:?}", exif);

        let value: i64 = exif.try_into().ok()?;

        Some(Color { value })
    }
}
