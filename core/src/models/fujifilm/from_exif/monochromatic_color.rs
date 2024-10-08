use crate::exif::{ExifData, FindExifData, FromExifData};
use crate::models::fujifilm::{MonochromaticColor, MonochromaticColorShift};
use log::debug;

impl FromExifData for MonochromaticColor {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        // TODO: Adjust Toning for X-Trans III Cameras (X-T3, X-T30, etc.)
        let exif_adjustment = data.find("BWAdjustment")?;
        let exif_magenta = data.find("BWMagentaGreen")?;

        debug!("MonochromaticColor::from_exif: WC {:?}", exif_adjustment);
        debug!("MonochromaticColor::from_exif: MG {:?}", exif_magenta);

        let wc: i64 = exif_adjustment.try_into().ok()?;
        let mg: i64 = exif_magenta.try_into().ok()?;

        Some(MonochromaticColor::ColorShift {
            shift: MonochromaticColorShift { wc, mg },
        })
    }
}
