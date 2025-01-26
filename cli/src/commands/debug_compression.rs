use crate::{
    photo::process::{Error as ProcessPhotoError, ProcessedPhoto},
    utils::is_valid_extension,
};
use image::error::ImageError;
use log::debug;
use snafu::prelude::*;
use std::path::Path;

pub async fn debug_compression(src: &Path) -> Result<(), Error> {
    if !is_valid_extension(src) {
        return Err(Error::Extension {
            path: src.to_str().unwrap().to_string(),
        });
    }

    let debug_dir = std::path::Path::new("__debug_compression__");
    if !std::fs::exists(debug_dir).unwrap() {
        std::fs::create_dir(debug_dir).unwrap();
    }

    debug!("Opening photo");
    let img = image::open(src).context(OpenSnafu)?;

    let mut processed: Vec<ProcessedPhoto> = Vec::new();

    debug!("Processing HD");
    processed.push(ProcessedPhoto::build_hd(&img).context(ProcessSnafu)?);
    debug!("Processing MD");
    processed.push(ProcessedPhoto::build_md(&img).context(ProcessSnafu)?);
    debug!("Processing SM");
    processed.push(ProcessedPhoto::build_sm(&img).context(ProcessSnafu)?);

    for photo in processed {
        let target_jpeg = debug_dir
            .join(photo.size.to_string())
            .with_extension("jpeg");
        let target_webp = debug_dir
            .join(photo.size.to_string())
            .with_extension("webp");

        std::fs::write(target_jpeg, &*photo.buffers.jpeg).expect("Failed to write JPEG");
        std::fs::write(target_webp, &*photo.buffers.webp).expect("Failed to write JPEG");
    }

    Ok(())
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Invalid file, wrong extension: {}", path))]
    Extension { path: String },

    #[snafu(display("Unable to open file: {}", source))]
    Open { source: ImageError },

    #[snafu(display("Failed to process photo: {}", source))]
    Process { source: ProcessPhotoError },
}
