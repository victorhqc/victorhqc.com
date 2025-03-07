use crate::models::exif_meta::Aperture;
use fuji::exif::{ExifData, FindExifData, FromExifData};
use log::trace;

impl FromExifData for Aperture {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        let exif = data.find("Aperture")?;

        trace!("Aperture::from_exif: {:?}", exif);

        let value: f64 = exif.try_into().ok()?;

        Some(Aperture(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_aperture() {
        let exif: Vec<ExifData> = vec![ExifData::new("Aperture", "5.6")];

        assert_eq!(Aperture::from_exif(&exif), Some(Aperture(5.6)));
    }

    #[test]
    fn it_does_not_parse_when_not_found() {
        let exif: Vec<ExifData> = vec![ExifData::new("Foo", "400")];

        assert_eq!(Aperture::from_exif(&exif), None);
    }
}
