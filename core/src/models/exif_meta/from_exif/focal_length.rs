use crate::models::exif_meta::FocalLength;
use fuji::exif::{ExifData, FindExifData, FromExifData};
use log::trace;
use once_cell::sync::Lazy;
use regex::Regex;

impl FromExifData for FocalLength {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        let focal_length_exif = data.find("FocalLength")?;
        let equivalent_exif = data.find("FocalLength35efl")?;

        trace!("FocalLength::from_exif: Value {:?}", focal_length_exif);
        trace!("FocalLength::from_exif: Equivalent {:?}", equivalent_exif);

        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+\.\d+").unwrap());

        let value = RE
            .captures_iter(focal_length_exif.value())
            .last()
            .map(|cap| cap.get(0).unwrap().as_str())?;

        let equivalent = RE
            .captures_iter(equivalent_exif.value())
            .last()
            .map(|cap| cap.get(0).unwrap().as_str())?;

        let value: f64 = value.parse::<f64>().ok()?;
        let eq_35mm: f64 = equivalent.parse::<f64>().ok()?;

        let crop_factor = eq_35mm / value;

        Some(FocalLength {
            value,
            eq_35mm,
            crop_factor,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_focal_length() {
        let exif: Vec<ExifData> = vec![
            ExifData::new("FocalLength", "23.0 mm"),
            ExifData::new("FocalLength35efl", "23.0 mm (35 mm equivalent: 35.0 mm)"),
        ];

        assert_eq!(
            FocalLength::from_exif(&exif),
            Some(FocalLength {
                value: 23.0,
                eq_35mm: 35.0,
                crop_factor: 1.5217391304347827,
            })
        );
    }

    #[test]
    fn it_does_not_parse_when_missing_data() {
        let exif: Vec<ExifData> = vec![ExifData::new("FocalLength", "23.0 mm")];
        assert_eq!(FocalLength::from_exif(&exif), None);

        let exif: Vec<ExifData> = vec![ExifData::new(
            "FocalLength35efl",
            "23.0 mm (35 mm equivalent: 35.0 mm)",
        )];
        assert_eq!(FocalLength::from_exif(&exif), None);
    }

    #[test]
    fn it_does_not_parse_when_not_found() {
        let exif: Vec<ExifData> = vec![
            ExifData::new("Foo", "23.0 mm"),
            ExifData::new("Foo2", "23.0 mm (35 mm equivalent: 35.0 mm)"),
        ];
        assert_eq!(FocalLength::from_exif(&exif), None);
    }
}
