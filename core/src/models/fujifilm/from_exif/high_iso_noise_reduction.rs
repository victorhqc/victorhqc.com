use crate::exif::{ExifData, FindExifData, FromExifData};
use crate::models::fujifilm::HighISONoiseReduction;
use log::debug;

impl FromExifData for HighISONoiseReduction {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        let exif = data.find("NoiseReduction")?;

        debug!("HighISONoiseReduction::from_exif: {:?}", exif);

        let value: i64 = exif.try_into().ok()?;

        Some(HighISONoiseReduction { value })
    }
}
