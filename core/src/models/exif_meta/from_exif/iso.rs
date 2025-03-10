use crate::models::exif_meta::Iso;
use fuji::exif::{ExifData, FindExifData, FromExifData};
use log::trace;

impl FromExifData for Iso {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        let exif = data.find("ISO")?;

        trace!("Iso::from_exif: {:?}", exif);

        let value: i64 = exif.try_into().ok()?;

        Some(Iso(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_iso() {
        let exif: Vec<ExifData> = vec![ExifData::new("ISO", "400")];

        assert_eq!(Iso::from_exif(&exif), Some(Iso(400)));
    }

    #[test]
    fn it_does_not_parse_when_not_found() {
        let exif: Vec<ExifData> = vec![ExifData::new("Foo", "400")];

        assert_eq!(Iso::from_exif(&exif), None);
    }
}
