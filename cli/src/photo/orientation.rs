use std::path::Path;

use core_victorhqc_com::models::photo::Orientation;
use image::ImageReader;
use snafu::prelude::*;

pub fn get_orientation(path: &Path) -> Result<Orientation, OrientationError> {
    let img = ImageReader::open(path)
        .context(ImageReaderSnafu)?
        .decode()
        .context(ImageDecodeSnafu)?;

    let (width, height) = (img.width(), img.height());

    Ok(if width > height {
        Orientation::Landscape
    } else {
        Orientation::Portrait
    })
}

#[derive(Debug, Snafu)]
pub enum OrientationError {
    #[snafu(display("Failed to read image: {}", source))]
    ImageReader { source: std::io::Error },

    #[snafu(display("Failed to decode image: {}", source))]
    ImageDecode { source: image::ImageError },
}
