use crate::models::exif_meta::ExposureCompensation;
use fuji::exif::{ExifData, FindExifData, FromExifData};
use log::trace;

impl FromExifData for ExposureCompensation {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        let exif = data.find("ExposureCompensation")?;

        trace!("ExposureCompensation::from_exif: {:?}", exif);

        let value: f64 = exif.try_into().ok()?;

        Some(ExposureCompensation(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_exposure_compensation() {
        let exif: Vec<ExifData> = vec![ExifData::new("ExposureCompensation", "+0.67")];

        assert_eq!(
            ExposureCompensation::from_exif(&exif),
            Some(ExposureCompensation(0.67))
        );
    }

    #[test]
    fn it_parses_exposure_compensation_negative_numbers() {
        let exif: Vec<ExifData> = vec![ExifData::new("ExposureCompensation", "-1.67")];

        assert_eq!(
            ExposureCompensation::from_exif(&exif),
            Some(ExposureCompensation(-1.67))
        );
    }

    #[test]
    fn it_does_not_parse_when_not_found() {
        let exif: Vec<ExifData> = vec![ExifData::new("Foo", "400")];

        assert_eq!(ExposureCompensation::from_exif(&exif), None);
    }
}
