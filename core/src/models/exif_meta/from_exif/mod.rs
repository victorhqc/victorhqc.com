mod aperture;
mod city;
mod date_taken;
mod exposure_compensation;
mod focal_length;
mod iso;
mod maker;
mod photography_details;
mod rating;
mod shutter_speed;

use fuji::exif::ExifData;
pub use photography_details::*;

pub trait TryFromExifData {
    type Error;

    fn try_from_exif(data: &[ExifData]) -> Result<Self, Self::Error>
    where
        Self: Sized;
}
