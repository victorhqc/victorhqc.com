use crate::exif::{ExifData, FindExifData, FromExifData};
use crate::models::fujifilm::Clarity;
use log::debug;

impl FromExifData for Clarity {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        let exif = data.find("Clarity")?;

        debug!("Clarity::from_exif: \"{}\"", exif);

        let value: i64 = if let Ok(n) = exif.try_into() {
            n
        } else {
            return None;
        };

        Some(Clarity { value })
    }
}
