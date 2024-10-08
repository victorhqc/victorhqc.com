use crate::exif::{ExifData, FindExifData, FromExifData};
use crate::models::fujifilm::ToneCurve;
use log::debug;

impl FromExifData for ToneCurve {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        let h_exif = data.find("HighlightTone")?;
        let s_exif = data.find("ShadowTone")?;

        debug!("ToneCurve::from_exif: H {:?}", h_exif);
        debug!("ToneCurve::from_exif: S {:?}", s_exif);

        let highlights: f64 = h_exif.try_into().ok()?;
        let shadows: f64 = s_exif.try_into().ok()?;

        Some(ToneCurve {
            highlights,
            shadows,
        })
    }
}
