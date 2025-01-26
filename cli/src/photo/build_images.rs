use super::process::{Error as ProcessPhotoError, ProcessedBuffers, ProcessedPhoto};
use crate::utils::is_valid_extension;
use console::Emoji;
use core_victorhqc_com::aws::image_size::ImageSize;
use image::error::ImageError;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use log::trace;
use snafu::prelude::*;
use std::{
    any::Any,
    path::Path,
    sync::mpsc::{Receiver, SendError, Sender},
    thread::{self, JoinHandle},
    time::Duration,
};

pub struct ImageBuffers {
    pub hd: ProcessedBuffers,
    pub md: ProcessedBuffers,
    pub sm: ProcessedBuffers,
}

pub enum ImageProcess {
    Opened,
    Processed(ProcessedPhoto),
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

            let processed = ProcessedPhoto::build_hd(&img_hd).context(ProcessSnafu)?;

            trace!("HD Image Processing completed");

            tx_hd
                .send(ImageProcess::Processed(processed))
                .context(ThreadSendSnafu)
        });

        let img_md = img.clone();
        let tx_md = tx.clone();
        let handle_md: BuildHandle = thread::spawn(move || {
            trace!("Building MD Image");

            let processed = ProcessedPhoto::build_md(&img_md).context(ProcessSnafu)?;

            trace!("MD Image Processing completed");

            tx_md
                .send(ImageProcess::Processed(processed))
                .context(ThreadSendSnafu)
        });

        let img_sm = img.clone();
        let handle_sm: BuildHandle = thread::spawn(move || {
            trace!("Building SM Image");
            let processed = ProcessedPhoto::build_sm(&img_sm).context(ProcessSnafu)?;

            trace!("SM Image Processing completed");

            tx.send(ImageProcess::Processed(processed))
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
    let mut hd: Option<ProcessedBuffers> = None;
    let mut md: Option<ProcessedBuffers> = None;
    let mut sm: Option<ProcessedBuffers> = None;

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
                        hd = Some(data.buffers)
                    }
                    ImageSize::Md => {
                        md_pb.set_style(s_done.clone());
                        md_pb.set_prefix("[3/4] ✓");
                        md_pb.finish_with_message(format!(
                            "{} MD Image Processing Finished",
                            PACKAGE
                        ));
                        md = Some(data.buffers)
                    }
                    ImageSize::Sm => {
                        sm_pb.set_style(s_done.clone());
                        sm_pb.set_prefix("[2/4] ✓");
                        sm_pb.finish_with_message(format!(
                            "{} SM Image Processing Finished",
                            PACKAGE
                        ));
                        sm = Some(data.buffers)
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

    #[snafu(display("Failed to process photo: {}", source))]
    Process { source: ProcessPhotoError },

    #[snafu(display("Failed to send data through TX: {}", source))]
    ThreadSend { source: SendError<ImageProcess> },

    #[snafu(display("Something went wrong while making images"))]
    MissingData,

    #[snafu(display("Thread panicked: {:?}", err))]
    ThreadPanic { err: Box<dyn Any + Send> },
}
