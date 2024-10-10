use crate::exif::{ExifData, FindExifData, FromExifData};
use crate::models::exif_meta::{
    Aperture, ExposureCompensation, FocalLength, Iso, Maker, PhotographyDetails,
};
use log::debug;

impl FromExifData for PhotographyDetails {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
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

    #[test]
    fn it_parses_photography_details_from_exif() {
        let exif: Vec<ExifData> = vec![
            ExifData::new("Model", "X-T5"),
            ExifData::new("LensModel", "XF23mmF1.4 R LM WR"),
            ExifData::new("Aperture", "2.8"),
            ExifData::new("ExposureCompensation", "+0.67"),
            ExifData::new("FocalLength", "23.0 mm"),
            ExifData::new("FocalLength35efl", "23.0 mm (35 mm equivalent: 35.0 mm)"),
            ExifData::new("ISO", "800"),
            ExifData::new("Make", "FUJIFILM"),
        ];

        assert_eq!(
            PhotographyDetails::from_exif(&exif),
            Some(PhotographyDetails {
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
