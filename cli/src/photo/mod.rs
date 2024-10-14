use crate::utils::is_valid_extension;
use image::{
    codecs::jpeg::JpegEncoder, error::ImageError, imageops::FilterType::Lanczos3, DynamicImage,
    GenericImageView,
};
use log::debug;
use snafu::prelude::*;
use std::io::Cursor;
use std::path::Path;

pub struct ImagesToUpload {
    pub hd: Vec<u8>,
    pub md: Vec<u8>,
    pub sm: Vec<u8>,
}

/// Creates buffers based on a path with a valid JPG image.
/// These buffers do not have exif metadata and have the following sizes:
/// - HD: 40% of the original image with JPEG quality of 80
/// - MD: 25% of the original image with JPEG quality of 75
/// - SM: 10% of the original image with JPEG quality of 30
pub async fn images_to_upload(path: &Path) -> Result<ImagesToUpload, Error> {
    if !is_valid_extension(path) {
        return Err(Error::Extension {
            path: path.to_str().unwrap().to_string(),
        });
    }

    let img = image::open(path).context(OpenSnafu)?;

    debug!("Building HD Image");
    let hd_img = resize(img.clone(), 0.4);
    let hd_img = compress(hd_img, 80)?;

    debug!("Building MD Image");
    let md_img = resize(img.clone(), 0.25);
    let md_img = compress(md_img, 75)?;

    debug!("Building SM Image");
    let sm_img = resize(img.clone(), 0.1);
    let sm_img = compress(sm_img, 30)?;

    Ok(ImagesToUpload {
        hd: hd_img,
        md: md_img,
        sm: sm_img,
    })
}

fn resize(img: DynamicImage, percentage: f32) -> DynamicImage {
    // let dimensions = img.dimensions();
    let (width, height) = img.dimensions();

    let width = (width as f32 * percentage).round() as u32;
    let height = (height as f32 * percentage).round() as u32;

    img.resize_exact(width, height, Lanczos3)
}

fn compress(img: DynamicImage, quality: u8) -> Result<Vec<u8>, Error> {
    let mut buffer: Vec<u8> = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);

    let encoder = JpegEncoder::new_with_quality(&mut cursor, quality);
    img.write_with_encoder(encoder).context(JpegSnafu)?;

    Ok(buffer)
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Invalid file, wrong extension: {}", path))]
    Extension { path: String },

    #[snafu(display("Unable to open file: {:?}", source))]
    Open { source: ImageError },

    #[snafu(display("Failed to encode JPEG: {:?}", source))]
    Jpeg { source: ImageError },
}
