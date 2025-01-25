use crate::utils::is_valid_extension;
use console::Emoji;
use core_victorhqc_com::aws::image_size::ImageSize;
use image::{
    codecs::jpeg::JpegEncoder, error::ImageError, imageops::FilterType::Lanczos3, DynamicImage,
    GenericImageView,
};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use log::trace;
use snafu::prelude::*;
use std::{
    any::Any,
    io::Cursor,
    path::Path,
    sync::mpsc::{Receiver, SendError, Sender},
    thread::{self, JoinHandle},
    time::Duration,
};

pub struct ImageBuffers {
    pub hd: ImageBuffer,
    pub md: ImageBuffer,
    pub sm: ImageBuffer,
}

pub struct ImageBuffer {
    pub jpeg: Vec<u8>,
    pub webp: Vec<u8>,
}

pub enum ImageProcess {
    Opened,
    Processed(ImgData),
}

pub struct ImgData {
    size: ImageSize,
    buffer: ImageBuffer,
}

pub type BuildHandle = JoinHandle<Result<(), Error>>;
pub type MainHandle = JoinHandle<Result<(BuildHandle, BuildHandle, BuildHandle), Error>>;

#[cfg(target_os = "windows")]
static PACKAGE: Emoji<'_, '_> = Emoji("📦", "");
#[cfg(not(target_os = "windows"))]
static PACKAGE: Emoji<'_, '_> = Emoji("📦 ", "");
#[cfg(target_os = "windows")]
static DRAWER: Emoji<'_, '_> = Emoji("🗃️", "");
#[cfg(not(target_os = "windows"))]
static DRAWER: Emoji<'_, '_> = Emoji("🗃️  ", "");

/// Creates buffers based on a path with a valid JPG image.
/// These buffers do not have exif metadata and have the following sizes:
/// - HD: 40% of the original image with JPEG quality of 80 and a WEBP (lossy) with 80% quality
/// - MD: 25% of the original image with JPEG quality of 75 and a WEBP (lossy) with 75% quality
/// - SM: 15% of the original image with JPEG quality of 70 and a WEBP (lossy) with 30% quality
pub fn start_build(path: &Path, tx: Sender<ImageProcess>) -> Result<MainHandle, Error> {
    if !is_valid_extension(path) {
        return Err(Error::Extension {
            path: path.to_str().unwrap().to_string(),
        });
    }

    let p = path.to_str().unwrap().to_string();
    let main_handle: MainHandle = thread::spawn(move || {
        trace!("Opening Image");
        let img = image::open(Path::new(&p)).context(OpenSnafu)?;
        tx.send(ImageProcess::Opened).context(ThreadSendSnafu)?;

        let img_hd = img.clone();
        let tx_hd = tx.clone();
        let handle_hd: BuildHandle = thread::spawn(move || {
            trace!("Building HD Image");

            let img_hd = resize(img_hd, 0.4);

            let webp_hd = convert_to_webp(&img_hd, 80f32)?;
            let img_hd = compress(img_hd, 80)?;

            trace!("HD Image Processing completed");

            tx_hd
                .send(ImageProcess::Processed(ImgData {
                    size: ImageSize::Hd,
                    buffer: ImageBuffer {
                        jpeg: img_hd,
                        webp: webp_hd,
                    },
                }))
                .context(ThreadSendSnafu)
        });

        let img_md = img.clone();
        let tx_md = tx.clone();
        let handle_md: BuildHandle = thread::spawn(move || {
            trace!("Building MD Image");
            let img_md = resize(img_md, 0.25);

            let webp_md = convert_to_webp(&img_md, 75f32)?;
            let img_md = compress(img_md, 75)?;

            trace!("MD Image Processing completed");

            tx_md
                .send(ImageProcess::Processed(ImgData {
                    size: ImageSize::Md,
                    buffer: ImageBuffer {
                        jpeg: img_md,
                        webp: webp_md,
                    },
                }))
                .context(ThreadSendSnafu)
        });

        let img_sm = img.clone();
        let handle_sm: BuildHandle = thread::spawn(move || {
            trace!("Building SM Image");
            let img_sm = resize(img_sm, 0.15);

            let webp_sm = convert_to_webp(&img_sm, 70f32)?;
            let img_sm = compress(img_sm, 70)?;

            trace!("SM Image Processing completed");

            tx.send(ImageProcess::Processed(ImgData {
                size: ImageSize::Sm,
                buffer: ImageBuffer {
                    jpeg: img_sm,
                    webp: webp_sm,
                },
            }))
            .context(ThreadSendSnafu)
        });

        Ok((handle_hd, handle_md, handle_sm))
    });

    Ok(main_handle)
}

pub fn finish_build(
    rx: Receiver<ImageProcess>,
    main_handle: MainHandle,
) -> Result<ImageBuffers, Error> {
    let mut hd: Option<ImageBuffer> = None;
    let mut md: Option<ImageBuffer> = None;
    let mut sm: Option<ImageBuffer> = None;

    let m = MultiProgress::new();
    let s = ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")
        .unwrap()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");
    let s_done = ProgressStyle::with_template("{prefix:.bold.dim} {wide_msg}").unwrap();

    let opened_pb = build_loader(&m, &s, format!("{} Opening Image...", DRAWER), 1);
    let sm_pb = build_loader(&m, &s, format!("{} Processing SM Image...", PACKAGE), 2);
    let md_pb = build_loader(&m, &s, format!("{} Processing MD Image...", PACKAGE), 3);
    let hd_pb = build_loader(&m, &s, format!("{} Processing HD Image...", PACKAGE), 4);

    for process in rx {
        match process {
            ImageProcess::Opened => {
                opened_pb.set_style(s_done.clone());
                opened_pb.set_prefix("[1/4] ✓");
                opened_pb.finish_with_message(format!("{} Image Opened", DRAWER));
            }
            ImageProcess::Processed(data) => {
                match data.size {
                    ImageSize::Hd => {
                        hd_pb.set_style(s_done.clone());
                        hd_pb.set_prefix("[4/4] ✓");
                        hd_pb.finish_with_message(format!(
                            "{} HD Image Processing Finished",
                            PACKAGE
                        ));
                        hd = Some(data.buffer)
                    }
                    ImageSize::Md => {
                        md_pb.set_style(s_done.clone());
                        md_pb.set_prefix("[3/4] ✓");
                        md_pb.finish_with_message(format!(
                            "{} MD Image Processing Finished",
                            PACKAGE
                        ));
                        md = Some(data.buffer)
                    }
                    ImageSize::Sm => {
                        sm_pb.set_style(s_done.clone());
                        sm_pb.set_prefix("[2/4] ✓");
                        sm_pb.finish_with_message(format!(
                            "{} SM Image Processing Finished",
                            PACKAGE
                        ));
                        sm = Some(data.buffer)
                    }
                };
            }
        }
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

    m.clear().unwrap();

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

fn convert_to_webp(img: &DynamicImage, quality: f32) -> Result<Vec<u8>, Error> {
    let encoder = webp::Encoder::from_image(img).map_err(|e| Error::Webp {
        error: e.to_string(),
    })?;
    let webp: webp::WebPMemory = encoder.encode(quality);

    Ok(webp.to_vec())
}

fn build_loader(
    m: &MultiProgress,
    spinner_style: &ProgressStyle,
    message: String,
    no: u8,
) -> ProgressBar {
    let pb = m.add(ProgressBar::new_spinner());
    pb.enable_steady_tick(Duration::from_millis(50));
    pb.set_style(spinner_style.clone());
    pb.set_prefix(format!("[{}/4]", no));
    pb.set_message(message);

    pb
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Invalid file, wrong extension: {}", path))]
    Extension { path: String },

    #[snafu(display("Unable to open file: {}", source))]
    Open { source: ImageError },

    #[snafu(display("Failed to encode JPEG: {}", source))]
    Jpeg { source: ImageError },

    #[snafu(display("Failed to encode WEBP: {}", error))]
    Webp { error: String },

    #[snafu(display("Failed to send data through TX: {}", source))]
    ThreadSend { source: SendError<ImageProcess> },

    #[snafu(display("Something went wrong while making images"))]
    MissingData,

    #[snafu(display("Thread panicked: {:?}", err))]
    ThreadPanic { err: Box<dyn Any + Send> },
}
