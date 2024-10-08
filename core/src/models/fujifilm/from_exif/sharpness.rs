use crate::exif::{ExifData, FindExifData, FromExifData};
use crate::models::fujifilm::Sharpness;
use log::debug;

impl FromExifData for Sharpness {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        let exif = data.find("Sharpness")?;

        debug!("Sharpness::from_exif: {:?}", exif);

        let value: i64 = exif.try_into().ok()?;

        Some(Sharpness { value })
    }
}
