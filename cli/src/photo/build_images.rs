use crate::utils::is_valid_extension;
use core_victorhqc_com::aws::photo::ImageSize;
use image::{
    codecs::jpeg::JpegEncoder, error::ImageError, imageops::FilterType::Lanczos3, DynamicImage,
    GenericImageView,
};
use log::debug;
use snafu::prelude::*;
use std::{
    any::Any,
    io::Cursor,
    path::Path,
    sync::mpsc::{Receiver, SendError, Sender},
    thread::{self, JoinHandle},
};

pub struct ImageBuffers {
    pub hd: Vec<u8>,
    pub md: Vec<u8>,
    pub sm: Vec<u8>,
}

pub type ImgData = (ImageSize, Vec<u8>);

pub type BuildHandle = JoinHandle<Result<(), Error>>;
pub type MainHandle = JoinHandle<Result<(BuildHandle, BuildHandle, BuildHandle), Error>>;

/// Creates buffers based on a path with a valid JPG image.
/// These buffers do not have exif metadata and have the following sizes:
/// - HD: 40% of the original image with JPEG quality of 80
/// - MD: 25% of the original image with JPEG quality of 75
/// - SM: 10% of the original image with JPEG quality of 30
pub fn start_build(
    path: &Path,
    tx: Sender<ImgData>
) -> Result<MainHandle, Error> {
    if !is_valid_extension(path) {
        return Err(Error::Extension {
            path: path.to_str().unwrap().to_string(),
        });
    }

    let p = path.to_str().unwrap().to_string();
    let main_handle: MainHandle = thread::spawn(move || {
        debug!("Opening Image");
        let img = image::open(Path::new(&p)).context(OpenSnafu)?;

        let img_hd = img.clone();
        let tx_hd = tx.clone();
        let handle_hd: BuildHandle = thread::spawn(move || {
            debug!("Building HD Image");
            let img_hd = resize(img_hd, 0.4);
            let img_hd = compress(img_hd, 80)?;
            debug!("HD Image Processing completed");

            tx_hd.send((ImageSize::Hd, img_hd)).context(ThreadSendSnafu)
        });

        let img_md = img.clone();
        let tx_md = tx.clone();
        let handle_md: BuildHandle = thread::spawn(move || {
            debug!("Building MD Image");
            let img_md = resize(img_md, 0.25);
            let img_md = compress(img_md, 75)?;
            debug!("MD Image Processing completed");

            tx_md.send((ImageSize::Md, img_md)).context(ThreadSendSnafu)
        });

        let img_sm = img.clone();
        let handle_sm: BuildHandle = thread::spawn(move || {
            debug!("Building SM Image");
            let img_sm = resize(img_sm, 0.1);
            let img_sm = compress(img_sm, 30)?;
            debug!("SM Image Processing completed");

            tx.send((ImageSize::Sm, img_sm)).context(ThreadSendSnafu)
        });

        Ok((handle_hd, handle_md, handle_sm))
    });

    Ok(main_handle)
}

pub fn finish_build(rx: Receiver<ImgData>, main_handle: MainHandle) -> Result<ImageBuffers, Error> {
    let mut hd: Option<Vec<u8>> = None;
    let mut md: Option<Vec<u8>> = None;
    let mut sm: Option<Vec<u8>> = None;

    for (size, img) in rx {
        match size {
            ImageSize::Hd => hd = Some(img),
            ImageSize::Md => md = Some(img),
            ImageSize::Sm => sm = Some(img),
        };
    }

    let (hd_handle, md_handle, sm_handle) = main_handle
        .join()
        .map(|r| match r {
            Ok((handle_hd, handle_md, handle_sm)) => {
                let hd = handle_hd
                    .join()
                    .map_err(|e| Error::ThreadPanic { err: e })?;
                let md = handle_md
                    .join()
                    .map_err(|e| Error::ThreadPanic { err: e })?;
                let sm = handle_sm
                    .join()
                    .map_err(|e| Error::ThreadPanic { err: e })?;
                Ok((hd, md, sm))
            }
            Err(err) => Err(err),
        })
        .map_err(|e| Error::ThreadPanic { err: e })??;
    hd_handle?;
    md_handle?;
    sm_handle?;

    if let (Some(hd), Some(md), Some(sm)) = (hd, md, sm) {
        Ok(ImageBuffers { hd, md, sm })
    } else {
        Err(Error::MissingData)
    }
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

    #[snafu(display("Failed to send data through TX: {:?}", source))]
    ThreadSend {
        source: SendError<(ImageSize, Vec<u8>)>,
    },

    #[snafu(display("Something went wrong while making images"))]
    MissingData,

    #[snafu(display("Thread panicked: {:?}", err))]
    ThreadPanic { err: Box<dyn Any + Send> },
}
