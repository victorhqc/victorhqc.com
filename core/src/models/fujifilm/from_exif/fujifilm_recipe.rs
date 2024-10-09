use crate::exif::{ExifData, FromExifData};
use crate::models::fujifilm::{
    builder::SettingsBuilder, FilmSimulation, FujifilmRecipe, TransSensor,
};

impl FromExifData for FujifilmRecipe {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        let film_simulation = FilmSimulation::from_exif(data)?;
        let sensor = TransSensor::from_exif(data)?;
        let builder = SettingsBuilder::from_exif(data)?;
        let settings = sensor.settings(builder);

        Some(FujifilmRecipe {
            id: String::from("UNKNOWN"),
            name: String::from("UNKNOWN"),
            src: String::from("UNKNOWN"),
            film_simulation,
            sensor,
            settings,
        })
    }
}
