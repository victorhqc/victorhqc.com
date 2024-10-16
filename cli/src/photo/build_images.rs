use crate::utils::is_valid_extension;
use core_victorhqc_com::aws::photo::ImageSize;
use image::{
    codecs::jpeg::JpegEncoder, error::ImageError, imageops::FilterType::Lanczos3, DynamicImage,
    GenericImageView,
};
use log::debug;
use snafu::prelude::*;
use std::{io::Cursor, path::Path};
use tokio::task::{JoinError, JoinHandle};

pub struct ImageBuffers {
    pub hd: Vec<u8>,
    pub md: Vec<u8>,
    pub sm: Vec<u8>,
}

type ImgData = (ImageSize, Vec<u8>);

pub type FutureHandle = JoinHandle<Result<ImgData, Error>>;
pub type MainHandle = JoinHandle<Result<(ImgData, ImgData, ImgData), Error>>;

/// Creates buffers based on a path with a valid JPG image.
/// These buffers do not have exif metadata and have the following sizes:
/// - HD: 40% of the original image with JPEG quality of 80
/// - MD: 25% of the original image with JPEG quality of 75
/// - SM: 10% of the original image with JPEG quality of 30
pub fn start_build(
    path: &Path,
    // tx: Sender<(ImgData, ImgData, ImgData)>,
) -> Result<MainHandle, Error> {
    if !is_valid_extension(path) {
        return Err(Error::Extension {
            path: path.to_str().unwrap().to_string(),
        });
    }

    let p = path.to_str().unwrap().to_string();
    let main_handle: MainHandle = tokio::spawn(async move {
        debug!("Opening Image");
        let img = image::open(Path::new(&p)).context(OpenSnafu)?;

        let img_hd = img.clone();
        let future_hd: FutureHandle = tokio::spawn(async move {
            debug!("Building HD Image");
            let img_hd = resize(img_hd, 0.4);
            let img_hd = compress(img_hd, 80)?;
            debug!("HD Image Processing completed");

            Ok((ImageSize::Hd, img_hd))
        });

        let img_md = img.clone();
        let future_md: FutureHandle = tokio::spawn(async move {
            debug!("Building MD Image");
            let img_md = resize(img_md, 0.25);
            let img_md = compress(img_md, 75)?;
            debug!("MD Image Processing completed");

            Ok((ImageSize::Md, img_md))
        });

        let img_sm = img.clone();
        let future_sm: FutureHandle = tokio::spawn(async move {
            debug!("Building SM Image");
            let img_sm = resize(img_sm, 0.1);
            let img_sm = compress(img_sm, 30)?;
            debug!("SM Image Processing completed");

            Ok((ImageSize::Sm, img_sm))
        });

        let (hd, md, sm) = futures::join!(future_hd, future_md, future_sm);

        let hd = hd.context(ImageFutureSnafu)??;
        let md = md.context(ImageFutureSnafu)??;
        let sm = sm.context(ImageFutureSnafu)??;

        Ok((hd, md, sm))
    });

    Ok(main_handle)
}

pub async fn finish_build(main_handle: MainHandle) -> Result<ImageBuffers, Error> {
    let ((_, hd), (_, md), (_, sm)) = main_handle.await.context(ImageFutureSnafu)??;

    Ok(ImageBuffers { hd, md, sm })
}

fn resize(img: DynamicImage, percentage: f32) -> DynamicImage {
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

    #[snafu(display("Failed to execute future for Image Processing: {:?}", source))]
    ImageFuture { source: JoinError },
}
