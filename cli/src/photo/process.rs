use core::f32;
use core_victorhqc_com::aws::image_size::ImageSize;
use image::{
    codecs::jpeg::JpegEncoder, error::ImageError, imageops::FilterType::Lanczos3, DynamicImage,
    GenericImageView,
};
use log::debug;
use snafu::prelude::*;
use std::io::Cursor;

pub struct ProcessedPhoto {
    pub size: ImageSize,
    pub buffers: ProcessedBuffers,
}

pub struct ProcessedBuffers {
    pub jpeg: Vec<u8>,
    pub webp: Vec<u8>,
}

impl ProcessedPhoto {
    pub fn build_hd(img: &DynamicImage) -> Result<Self, Error> {
        Self::build(img, ImageSize::Hd, 1080, 75f32)
    }

    pub fn build_md(img: &DynamicImage) -> Result<Self, Error> {
        Self::build(img, ImageSize::Md, 720, 75f32)
    }

    pub fn build_sm(img: &DynamicImage) -> Result<Self, Error> {
        Self::build(img, ImageSize::Sm, 480, 70f32)
    }

    fn build(
        img: &DynamicImage,
        size: ImageSize,
        wanted_height: i32,
        compression: f32,
    ) -> Result<Self, Error> {
        debug!("Resizing {} Image", size);
        let resized = resize_with_known_dimensions(img, wanted_height);

        debug!("Converting to Webp");
        let webp = convert_to_webp(&resized, compression)?;

        debug!("Converting to JPEG");
        let jpeg = compress(&resized, compression)?;

        Ok(ProcessedPhoto {
            size,
            buffers: ProcessedBuffers { webp, jpeg },
        })
    }
}

/// This Function will resize the image to a known height. However, the height changes depending on
/// the photo. The height will always be the small side of the rectangle, meaning that it will be
/// the regular height when it is in landscape, but in a portrait photo, the height would be
/// technically, the width. Why this way? To keep the image sizes consistant.
fn resize_with_known_dimensions(img: &DynamicImage, wanted_height: i32) -> DynamicImage {
    let (width, height) = img.dimensions();

    let is_landscape = width > height;

    debug!("Is landscape? {}", is_landscape);

    let current_height = if is_landscape { height } else { width };
    debug!("Current height: {}", current_height);

    // To figure out the wanted width we must calculate how much percentage the wanted height
    // represents compared with the current height. If the current height is 10800 and the wanted
    // height is 1080, then the percentage would be 10% (1080 * 100 / 10800) so we should calculate
    // the width as 10% of its current size.
    let percentage: f32 = (wanted_height as f32 * 100.00) / current_height as f32;
    debug!("Percentage to shrink: {}", percentage);

    resize(img, percentage / 100.00)
}

fn resize(img: &DynamicImage, percentage: f32) -> DynamicImage {
    let (width, height) = img.dimensions();

    let width = (width as f32 * percentage).round() as u32;
    let height = (height as f32 * percentage).round() as u32;

    debug!("New dimensions: {} height, {} width", height, width);

    img.resize_exact(width, height, Lanczos3)
}

fn compress(img: &DynamicImage, quality: f32) -> Result<Vec<u8>, Error> {
    let mut buffer: Vec<u8> = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);

    let quality: u8 = quality as u8;
    let encoder = JpegEncoder::new_with_quality(&mut cursor, quality);
    img.write_with_encoder(encoder).context(JpegSnafu)?;

    Ok(buffer)
}

fn convert_to_webp(img: &DynamicImage, quality: f32) -> Result<Vec<u8>, Error> {
    let encoder = webp::Encoder::from_image(img).map_err(|e| Error::Webp {
        error: e.to_string(),
    })?;
    let webp: webp::WebPMemory = encoder.encode(quality);

    Ok(webp.to_vec())
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to encode JPEG: {}", source))]
    Jpeg { source: ImageError },

    #[snafu(display("Failed to encode WEBP: {}", error))]
    Webp { error: String },
}
