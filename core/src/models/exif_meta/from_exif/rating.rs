use crate::exif::{ExifData, FindExifData, FromExifData};
use crate::models::exif_meta::Rating;
use log::trace;

impl FromExifData for Rating {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        let exif = data.find("Rating")?;

        trace!("Rating::from_exif: {:?}", exif);

        let value: i64 = exif.try_into().ok()?;

        Some(Rating(value as i8))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_rating() {
        let exif: Vec<ExifData> = vec![ExifData::new("Rating", "3")];

        assert_eq!(Rating::from_exif(&exif), Some(Rating(3)));
    }

    #[test]
    fn it_does_not_parse_when_not_found() {
        let exif: Vec<ExifData> = vec![ExifData::new("Foo", "400")];

        assert_eq!(Rating::from_exif(&exif), None);
    }
}
