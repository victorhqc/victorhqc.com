use crate::exif::{ExifData, FindExifData, FromExifData};
use crate::models::exif_meta::{
    Aperture, City, DateTaken, ExposureCompensation, FocalLength, Iso, Maker, PhotographyDetails,
    Rating,
};
use log::debug;

impl FromExifData for PhotographyDetails {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        let rating = Rating::from_exif(data).or_else(|| {
            debug!("Rating Missing");
            None
        })?;
        let city = City::from_exif(data).or_else(|| {
            debug!("City Missing");
            None
        });
        let date_taken = DateTaken::from_exif(data).or_else(|| {
            debug!("Date Taken Missing");
            None
        });
        let aperture = Aperture::from_exif(data).or_else(|| {
            debug!("Aperture Missing");
            None
        })?;
        let exposure_compensation = ExposureCompensation::from_exif(data).or_else(|| {
            debug!("Exposure Compensation Missing");
            None
        })?;
        let focal_length = FocalLength::from_exif(data).or_else(|| {
            debug!("Focal Length Missing");
            None
        })?;
        let iso = Iso::from_exif(data).or_else(|| {
            debug!("Iso Missing");
            None
        })?;
        let maker = Maker::from_exif(data).or_else(|| {
            debug!("Maker Missing");
            None
        })?;
        let lens_name = data.find("LensModel").map(|n| n.value().to_string());
        let camera_name = data.find("Model").or_else(|| {
            debug!("Camera Model Missing");
            None
        })?;

        Some(PhotographyDetails {
            rating,
            date_taken,
            city,
            camera_name: camera_name.value().to_string(),
            lens_name,
            aperture,
            exposure_compensation,
            focal_length,
            iso,
            maker,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::{Date, Month, OffsetDateTime, Time, UtcOffset};

    #[test]
    fn it_parses_photography_details_from_exif() {
        let exif: Vec<ExifData> = vec![
            ExifData::new("Rating", "3"),
            ExifData::new("City", "Berlin"),
            ExifData::new("DateTimeOriginal", "2024:09:12 18:55:14.13+02:00"),
            ExifData::new("Model", "X-T5"),
            ExifData::new("LensModel", "XF23mmF1.4 R LM WR"),
            ExifData::new("Aperture", "2.8"),
            ExifData::new("ExposureCompensation", "+0.67"),
            ExifData::new("FocalLength", "23.0 mm"),
            ExifData::new("FocalLength35efl", "23.0 mm (35 mm equivalent: 35.0 mm)"),
            ExifData::new("ISO", "800"),
            ExifData::new("Make", "FUJIFILM"),
        ];

        let date = Date::from_calendar_date(2024, Month::September, 12).unwrap();
        let time = Time::from_hms(18, 55, 14).unwrap();
        let offset = UtcOffset::from_hms(2, 0, 0).unwrap();
        let date_time = OffsetDateTime::new_in_offset(date, time, offset);

        assert_eq!(
            PhotographyDetails::from_exif(&exif),
            Some(PhotographyDetails {
                rating: Rating(3),
                city: Some(City("Berlin".to_string())),
                date_taken: Some(DateTaken(date_time)),
                camera_name: "X-T5".to_string(),
                lens_name: Some("XF23mmF1.4 R LM WR".to_string()),
                aperture: Aperture(2.8),
                exposure_compensation: ExposureCompensation(0.67),
                focal_length: FocalLength {
                    value: 23.0,
                    eq_35mm: 35.0,
                    crop_factor: 1.5217391304347827,
                },
                iso: Iso(800),
                maker: Maker::Fujifilm,
            })
        );
    }

    #[test]
    fn it_does_not_parse_if_any_property_is_missing() {
        let exif: Vec<ExifData> = vec![
            ExifData::new("Rating", "3"),
            ExifData::new("City", "Berlin"),
            ExifData::new("DateTimeOriginal", "2024:09:12 18:55:14.13+02:00"),
            ExifData::new("Model", "X-T5"),
            ExifData::new("LensModel", "XF23mmF1.4 R LM WR"),
            ExifData::new("ExposureCompensation", "+0.67"),
            ExifData::new("FocalLength", "23.0 mm"),
            ExifData::new("FocalLength35efl", "23.0 mm (35 mm equivalent: 35.0 mm)"),
            ExifData::new("ISO", "800"),
            ExifData::new("Make", "FUJIFILM"),
        ];

        assert_eq!(PhotographyDetails::from_exif(&exif), None,);
    }
}
