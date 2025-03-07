use crate::models::exif_meta::ShutterSpeed;
use fuji::exif::{ExifData, FindExifData, FromExifData};
use log::trace;

impl FromExifData for ShutterSpeed {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        let exif = data.find("ShutterSpeed")?;

        trace!("ShutterSpeed::from_exif: {:?}", exif);

        Some(ShutterSpeed(exif.value().to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_shutter_speed() {
        let exif: Vec<ExifData> = vec![ExifData::new("ShutterSpeed", "1/500")];

        assert_eq!(
            ShutterSpeed::from_exif(&exif),
            Some(ShutterSpeed("1/500".to_string()))
        );
    }

    #[test]
    fn it_does_not_parse_when_not_found() {
        let exif: Vec<ExifData> = vec![ExifData::new("NotShutterSpeed", "1/400")];

        assert_eq!(ShutterSpeed::from_exif(&exif), None);
    }
}
