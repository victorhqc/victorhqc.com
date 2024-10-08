use crate::exif::{ExifData, FindExifData, FromExifData};
use crate::models::fujifilm::Color;
use log::debug;

impl FromExifData for Color {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        let exif = data.find("Saturation")?;

        debug!("Color::from_exif: {:?}", exif);

        let value: i64 = exif.try_into().ok()?;

        Some(Color { value })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exif::ExifData;
    use crate::models::fujifilm::Color;

    #[test]
    fn it_parses_positive_numbers() {
        let exif: Vec<ExifData> = vec![ExifData::new("Saturation", "3")];

        assert_eq!(Color::from_exif(&exif), Some(Color { value: 3 }));
    }

    #[test]
    fn it_parses_negative_numbers() {
        let exif: Vec<ExifData> = vec![ExifData::new("Saturation", "-3")];

        assert_eq!(Color::from_exif(&exif), Some(Color { value: -3 }));
    }

    #[test]
    fn it_does_not_parse_when_not_found() {
        let exif: Vec<ExifData> = vec![ExifData::new("Foo", "3")];

        assert_eq!(Color::from_exif(&exif), None);
    }
}
