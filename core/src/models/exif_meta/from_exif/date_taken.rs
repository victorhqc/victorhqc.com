use crate::exif::{ExifData, FindExifData, FromExifData};
use crate::models::exif_meta::DateTaken;
use log::debug;

impl FromExifData for DateTaken {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        let exif = data.find("DateTimeOriginal")?;

        debug!("DateTaken::from_exif: {:?}", exif);

        // let value = OffsetDateTime::parse(exif.value());
        // let value: OffsetDateTime = exif.try_into().ok()?;

        Some(DateTaken(exif.value().to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_date_taken() {
        let exif: Vec<ExifData> = vec![ExifData::new(
            "DateTimeOriginal",
            "2024:09:12 18:55:14.13+02:00",
        )];

        assert_eq!(
            DateTaken::from_exif(&exif),
            Some(DateTaken("2024:09:12 18:55:14.13+02:00".to_string()))
        );
    }

    #[test]
    fn it_does_not_parse_when_not_found() {
        let exif: Vec<ExifData> = vec![ExifData::new("Foo", "400")];

        assert_eq!(DateTaken::from_exif(&exif), None);
    }
}
