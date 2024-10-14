use crate::exif::{ExifData, FindExifData, FromExifData};
use crate::models::exif_meta::Maker;
use log::trace;
use std::str::FromStr;

impl FromExifData for Maker {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        let exif = data.find("Make")?;

        trace!("Maker::from_exif: {:?}", exif);

        match Maker::from_str(exif.value()) {
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

        assert_eq!(Maker::from_exif(&exif), Some(Maker::Fujifilm));
    }

    #[test]
    fn it_parses_konica_maker() {
        let exif: Vec<ExifData> = vec![ExifData::new("Make", "KONICA")];

        assert_eq!(Maker::from_exif(&exif), Some(Maker::Konica));
    }

    #[test]
    fn it_parses_canon_maker() {
        let exif: Vec<ExifData> = vec![ExifData::new("Make", "CANON")];

        assert_eq!(Maker::from_exif(&exif), Some(Maker::Canon));
    }

    #[test]
    fn it_does_not_parse_when_not_found() {
        let exif: Vec<ExifData> = vec![ExifData::new("Foo", "FUJIFILM")];

        assert_eq!(Maker::from_exif(&exif), None);
    }
}
