use crate::exif::{ExifData, FindExifData, FromExifData};
use crate::models::exif_meta::DateTaken;
use log::trace;
use once_cell::sync::Lazy;
use regex::Regex;
use time::{Date, Month};

impl FromExifData for DateTaken {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        let exif = data.find("DateTimeOriginal")?;

        trace!("DateTaken::from_exif: {:?}", exif);

        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"(?<date>[0-9]{4}[-:][09]{2}[-:][0-9]{2}) (?<time>[0-9]{2}:[0-9]{2}:[0-9]{2}).*(?<offset>[+\-0-9]{3}:[0-9]{2})").unwrap()
        });
        static RE_DATE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"(?<year>[0-9]{4})[-:](?<month>[09]{2})[-:](?<day>[0-9]{2})").unwrap()
        });

        let caps = RE.captures(exif.value())?;
        let date = &caps["date"];

        let date_caps = RE_DATE.captures(date)?;
        let year = date_caps["year"].parse::<i32>().ok()?;
        let month = date_caps["month"].parse::<i32>().ok()?;
        let day = date_caps["day"].parse::<i32>().ok()?;

        let date = Date::from_calendar_date(year, Month::try_from(month as u8).unwrap(), day as u8)
            .ok()?;

        Some(DateTaken(date))
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
        let date = Date::from_calendar_date(2024, Month::September, 12).unwrap();

        assert_eq!(DateTaken::from_exif(&exif), Some(DateTaken(date)));
    }

    #[test]
    fn it_parses_with_negative_offset() {
        let exif: Vec<ExifData> = vec![ExifData::new(
            "DateTimeOriginal",
            "2024:09:12 18:55:14.13-07:30",
        )];
        let date = Date::from_calendar_date(2024, Month::September, 12).unwrap();

        assert_eq!(DateTaken::from_exif(&exif), Some(DateTaken(date)));
    }

    #[test]
    fn it_does_not_parse_when_not_found() {
        let exif: Vec<ExifData> = vec![ExifData::new("Foo", "400")];

        assert_eq!(DateTaken::from_exif(&exif), None);
    }
}
