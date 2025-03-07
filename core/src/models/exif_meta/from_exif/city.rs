use crate::models::exif_meta::City;
use fuji::exif::{ExifData, FindExifData, FromExifData};
use log::trace;

impl FromExifData for City {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        let exif = data.find("City")?;

        trace!("City::from_exif: {:?}", exif);

        Some(City(exif.value().to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_city() {
        let exif: Vec<ExifData> = vec![ExifData::new("City", "Berlin")];

        assert_eq!(City::from_exif(&exif), Some(City("Berlin".to_string())));
    }

    #[test]
    fn it_does_not_parse_when_not_found() {
        let exif: Vec<ExifData> = vec![ExifData::new("Foo", "Berlin")];

        assert_eq!(City::from_exif(&exif), None);
    }
}
