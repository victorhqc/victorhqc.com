use crate::photo::{
    aws::{remove, upload, Error as AWSError},
    build_images::{finish_build, start_build, Error as BuildImagesError, ImageProcess},
};
use core_victorhqc_com::sqlx::error::Error as SqlxError;
use core_victorhqc_com::{
    aws::S3,
    models::photo::{db::Error as PhotoDbError, Photo},
    sqlx::SqlitePool,
};
use log::debug;
use snafu::prelude::*;
use std::{path::Path, sync::mpsc};

pub async fn re_upload(pool: &SqlitePool, src: &Path, s3: &S3) -> Result<(), Error> {
    let mut conn = pool.begin().await.context(DBConnectSnafu)?;

    let photo = Photo::find_by_filename(&mut conn, src)
        .await
        .context(PathPhotoSnafu)?
        .context(MissingPhotoSnafu)?;

    let (tx, rx) = mpsc::channel::<ImageProcess>();

    debug!("Building Images to Re-upload");
    let main_handle = start_build(src, tx).context(BuildImagesSnafu)?;

    debug!("Removing Photos from AWS");
    remove(&photo, s3).await.context(RemoveSnafu)?;

    let buffers = finish_build(rx, main_handle).context(BuildImagesSnafu)?;
    debug!("About to upload to S3");
    upload(&photo, s3, buffers).await.context(UploadSnafu)?;
    debug!("Uploaded to S3");

    Ok(())
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to connect to db: {}", source))]
    DBConnect { source: SqlxError },

    #[snafu(display("Photo does not exist"))]
    MissingPhoto,

    #[snafu(display("Failed to check for photo by path: {}", source))]
    PathPhoto { source: PhotoDbError },

    #[snafu(display("Failed to build images: {}", source))]
    BuildImages { source: BuildImagesError },

    #[snafu(display("Failed to Upload {}", source))]
    Upload { source: AWSError },

    #[snafu(display("Failed to Remove {}", source))]
    Remove { source: AWSError },
}
