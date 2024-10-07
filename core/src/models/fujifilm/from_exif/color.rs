use crate::exif::{ExifData, FindExifData, FromExifData};
use crate::models::fujifilm::Color;

impl FromExifData for Color {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        let exif = data.find("Saturation")?;

        let value: i64 = if let Ok(n) = exif.try_into() {
            n
        } else {
            return None;
        };

        Some(Color { value })
    }
}
