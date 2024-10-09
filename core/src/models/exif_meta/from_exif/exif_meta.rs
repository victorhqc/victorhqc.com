use crate::exif::{ExifData, FindExifData, FromExifData};
use crate::models::exif_meta::ExifMeta;
use log::debug;

impl FromExifData for ExifMeta {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        todo!()
    }
}
