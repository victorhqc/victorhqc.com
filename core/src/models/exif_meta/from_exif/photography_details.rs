use crate::models::exif_meta::{
    Aperture, CameraMaker, City, DateTaken, ExposureCompensation, FocalLength, Iso, LensMaker,
    PhotographyDetails, Rating, ShutterSpeed, from_exif::TryFromExifData,
};
use fuji::exif::{ExifData, FindExifData, FromExifData};
use snafu::prelude::*;

impl TryFromExifData for PhotographyDetails {
    type Error = PhotographyDetailsError;

    fn try_from_exif(data: &[ExifData]) -> Result<Self, Self::Error> {
        let rating = Rating::from_exif(data)
            .or_else(|| {
                trace!("Rating Missing");
                None
            })
            .unwrap_or_default();
        let city = City::from_exif(data).or_else(|| {
            trace!("City Missing");
            None
        });
        let date_taken = DateTaken::from_exif(data).or_else(|| {
            trace!("Date Taken Missing");
            None
        });
        let aperture = Aperture::from_exif(data)
            .or_else(|| {
                trace!("Aperture Missing");
                None
            })
            .context(ApertureSnafu)?;
        let shutter_speed = ShutterSpeed::from_exif(data)
            .or_else(|| {
                trace!("Shutter Speed Missing");
                None
            })
            .context(ShutterSpeedSnafu)?;
        let exposure_compensation = ExposureCompensation::from_exif(data)
            .or_else(|| {
                trace!("Exposure Compensation Missing");
                None
            })
            .unwrap_or_default();
        let focal_length = FocalLength::from_exif(data)
            .or_else(|| {
                trace!("Focal Length Missing");
                None
            })
            .context(FocalLengthSnafu)?;
        let iso = Iso::from_exif(data)
            .or_else(|| {
                trace!("Iso Missing");
                None
            })
            .context(IsoSnafu)?;
        let camera_maker = CameraMaker::from_exif(data)
            .or_else(|| {
                trace!("Camera Maker Missing");
                None
            })
            .context(CameraMakerSnafu)?;
        let lens_maker = LensMaker::from_exif(data)
            .or_else(|| {
                trace!("Lens Maker Missing");
                None
            })
            .context(LensMakerSnafu)?;
        let lens_name = data.find("LensModel").map(|n| n.value().to_string());
        let camera_name = data
            .find("Model")
            .or_else(|| {
                trace!("Camera Model Missing");
                None
            })
            .context(CameraNameSnafu)?;

        Ok(PhotographyDetails {
            rating,
            date_taken,
            city,
            aperture,
            shutter_speed,
            camera_maker,
            camera_name: camera_name.value().to_string(),
            lens_maker,
            lens_name,
            exposure_compensation,
            focal_length,
            iso,
        })
    }
}

#[derive(Debug, Snafu, PartialEq)]
pub enum PhotographyDetailsError {
    #[snafu(display("Aperture value is missing"))]
    Aperture,

    #[snafu(display("Shutter Speed value is missing"))]
    ShutterSpeed,

    #[snafu(display("Focal Length value is missing"))]
    FocalLength,

    #[snafu(display("Iso value is missing"))]
    Iso,

    #[snafu(display("Camera Maker value is missing"))]
    CameraMaker,

    #[snafu(display("Camera Name value is missing"))]
    CameraName,

    #[snafu(display("Lens Maker value is missing"))]
    LensMaker,
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::{Date, Month};

    #[test]
    fn it_parses_photography_details_from_exif() {
        let exif: Vec<ExifData> = vec![
            ExifData::new("Rating", "3"),
            ExifData::new("City", "Berlin"),
            ExifData::new("DateTimeOriginal", "2024:09:12 18:55:14.13+02:00"),
            ExifData::new("Model", "X-T5"),
            ExifData::new("LensModel", "XF23mmF1.4 R LM WR"),
            ExifData::new("Aperture", "2.8"),
            ExifData::new("ShutterSpeed", "1/2000"),
            ExifData::new("ExposureCompensation", "+0.67"),
            ExifData::new("FocalLength", "23.0 mm"),
            ExifData::new("FocalLength35efl", "23.0 mm (35 mm equivalent: 35.0 mm)"),
            ExifData::new("ISO", "800"),
            ExifData::new("Make", "FUJIFILM"),
        ];

        let date = Date::from_calendar_date(2024, Month::September, 12).unwrap();

        assert_eq!(
            PhotographyDetails::try_from_exif(&exif),
            Ok(PhotographyDetails {
                rating: Rating(3),
                city: Some(City("Berlin".to_string())),
                date_taken: Some(DateTaken(date)),
                camera_name: "X-T5".to_string(),
                lens_name: Some("XF23mmF1.4 R LM WR".to_string()),
                aperture: Aperture(2.8),
                shutter_speed: ShutterSpeed("1/2000".to_string()),
                exposure_compensation: ExposureCompensation(0.67),
                focal_length: FocalLength {
                    value: 23.0,
                    eq_35mm: 35.0,
                    crop_factor: 1.5217391304347827,
                },
                iso: Iso(800),
                camera_maker: CameraMaker::Fujifilm,
                lens_maker: LensMaker::Unknown,
            })
        );
    }

    #[test]
    fn it_does_not_parse_if_aperture_is_missing() {
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

        assert_eq!(
            PhotographyDetails::try_from_exif(&exif),
            Err(PhotographyDetailsError::Aperture),
        );
    }
}
