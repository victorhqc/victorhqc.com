use crate::exif::{ExifData, FindExifData, FromExifData};
use crate::models::fujifilm::DynamicRange;
use log::debug;

impl FromExifData for DynamicRange {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        let exif = data.find("DynamicRangeSetting")?;

        debug!("DynamicRange::from_exif: {:?}", exif);

        match exif.value().to_lowercase().as_str() {
            "auto" => Some(DynamicRange::Auto),
            "100" => Some(DynamicRange::DR100),
            "200" => Some(DynamicRange::DR200),
            "400" => Some(DynamicRange::DR400),
            _ => None,
        }
    }
}
