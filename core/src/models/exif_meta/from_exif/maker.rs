use crate::exif::{ExifData, FindExifData, FromExifData};
use crate::models::exif_meta::{CameraMaker, LensMaker};
use log::trace;
use std::str::FromStr;

impl FromExifData for CameraMaker {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        let exif = data.find("Make")?;

        trace!("CameraMaker::from_exif: {:?}", exif);

        match Self::from_str(exif.value()) {
            Ok(maker) => Some(maker),
            Err(_) => None,
        }
    }
}

impl FromExifData for LensMaker {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        let exif = if let Some(exif) = data.find("LensMake") {
            exif
        } else {
            return Some(LensMaker::Unknown);
        };

        trace!("LensMaker::from_exif: {:?}", exif);

        match Self::from_str(exif.value()) {
            Ok(maker) => Some(maker),
            Err(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_fujifilm_maker() {
        let exif: Vec<ExifData> = vec![ExifData::new("Make", "FUJIFILM")];
        assert_eq!(CameraMaker::from_exif(&exif), Some(CameraMaker::Fujifilm));

        let exif: Vec<ExifData> = vec![ExifData::new("LensMake", "FUJIFILM")];
        assert_eq!(LensMaker::from_exif(&exif), Some(LensMaker::Fujifilm));
    }

    #[test]
    fn it_parses_konica_maker() {
        let exif: Vec<ExifData> = vec![ExifData::new("Make", "KONICA")];
        assert_eq!(CameraMaker::from_exif(&exif), Some(CameraMaker::Konica));

        let exif: Vec<ExifData> = vec![ExifData::new("LensMake", "KONICA")];
        assert_eq!(LensMaker::from_exif(&exif), Some(LensMaker::Konica));
    }

    #[test]
    fn it_parses_canon_maker() {
        let exif: Vec<ExifData> = vec![ExifData::new("Make", "CANON")];
        assert_eq!(CameraMaker::from_exif(&exif), Some(CameraMaker::Canon));

        let exif: Vec<ExifData> = vec![ExifData::new("LensMake", "CANON")];
        assert_eq!(LensMaker::from_exif(&exif), Some(LensMaker::Canon));
    }

    #[test]
    fn it_does_not_parse_when_not_found() {
        let exif: Vec<ExifData> = vec![ExifData::new("Foo", "FUJIFILM")];
        assert_eq!(CameraMaker::from_exif(&exif), None);

        let exif: Vec<ExifData> = vec![ExifData::new("Foo", "FUJIFILM")];
        assert_eq!(LensMaker::from_exif(&exif), Some(LensMaker::Unknown));

        let exif: Vec<ExifData> = vec![ExifData::new("LensMake", "WHO KNOWS")];
        assert_eq!(LensMaker::from_exif(&exif), None);
    }
}
