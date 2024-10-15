use crate::utils::is_valid_extension;
use core_victorhqc_com::aws::photo::ImageSize;
use image::{
    codecs::jpeg::JpegEncoder, error::ImageError, imageops::FilterType::Lanczos3, DynamicImage,
    GenericImageView,
};
use log::debug;
use snafu::prelude::*;
use std::any::Any;
use std::{
    io::Cursor,
    path::Path,
    sync::mpsc::{Receiver, SendError, Sender},
    thread,
};

pub struct ImageBuffers {
    pub hd: Vec<u8>,
    pub md: Vec<u8>,
    pub sm: Vec<u8>,
}

type ImgData = (ImageSize, Vec<u8>);

/// Creates buffers based on a path with a valid JPG image.
/// These buffers do not have exif metadata and have the following sizes:
/// - HD: 40% of the original image with JPEG quality of 80
/// - MD: 25% of the original image with JPEG quality of 75
/// - SM: 10% of the original image with JPEG quality of 30
pub fn build_images(
    path: &Path,
    (tx, rx): (Sender<ImgData>, Receiver<ImgData>),
) -> Result<ImageBuffers, Error> {
    if !is_valid_extension(path) {
        return Err(Error::Extension {
            path: path.to_str().unwrap().to_string(),
        });
    }

    debug!("Opening Image");
    let img = image::open(path).context(OpenSnafu)?;

    let img_hd = img.clone();
    let tx_hd = tx.clone();
    let handle_hd = thread::spawn(move || {
        debug!("Building HD Image");
        let img_hd = resize(img_hd, 0.4);
        let img_hd = match compress(img_hd, 80) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        debug!("HD Image Processing completed");

        tx_hd.send((ImageSize::Hd, img_hd)).context(ThreadSendSnafu)
    });

    let img_md = img.clone();
    let tx_md = tx.clone();
    let handle_md = thread::spawn(move || {
        debug!("Building MD Image");
        let img_md = resize(img_md, 0.25);
        let img_md = match compress(img_md, 75) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        debug!("MD Image Processing completed");

        tx_md.send((ImageSize::Md, img_md)).context(ThreadSendSnafu)
    });

    let img_sm = img.clone();
    let handle_sm = thread::spawn(move || {
        debug!("Building SM Image");
        let img_sm = resize(img_sm, 0.1);
        let img_sm = match compress(img_sm, 30) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        debug!("SM Image Processing completed");

        tx.send((ImageSize::Sm, img_sm)).context(ThreadSendSnafu)
    });

    handle_hd
        .join()
        .map_err(|e| Error::ThreadPanic { err: e })??;

    handle_md
        .join()
        .map_err(|e| Error::ThreadPanic { err: e })??;

    handle_sm
        .join()
        .map_err(|e| Error::ThreadPanic { err: e })??;

    let mut hd: (ImageSize, Option<Vec<u8>>) = (ImageSize::Hd, None);
    let mut md: (ImageSize, Option<Vec<u8>>) = (ImageSize::Md, None);
    let mut sm: (ImageSize, Option<Vec<u8>>) = (ImageSize::Sm, None);
    for (size, data) in rx {
        debug!("Received: {:?}", size);
        match size {
            ImageSize::Hd => {
                hd = (ImageSize::Hd, Some(data));
            }
            ImageSize::Md => {
                md = (ImageSize::Md, Some(data));
            }
            ImageSize::Sm => {
                sm = (ImageSize::Sm, Some(data));
            }
        }
    }

    if let ((_, Some(hd)), (_, Some(md)), (_, Some(sm))) = (hd, md, sm) {
        Ok(ImageBuffers { hd, md, sm })
    } else {
        Err(Error::MissingData)
    }
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

    #[snafu(display("Failed to send data through TX: {:?}", source))]
    ThreadSend {
        source: SendError<(ImageSize, Vec<u8>)>,
    },

    #[snafu(display("Thread panicked: {:?}", err))]
    ThreadPanic { err: Box<dyn Any + Send> },

    #[snafu(display("Something went wrong while making images"))]
    MissingData,
}
