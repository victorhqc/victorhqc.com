use crate::{
    exiftool,
    photo::{
        aws::{Error as AWSError, upload},
        build_images::{Error as BuildImagesError, ImageProcess, finish_build, start_build},
        orientation::{self, OrientationError},
    },
    utils::{GetFujifilmError, capture, get_some_fujifilm_recipe},
};
use console::Emoji;
use core_victorhqc_com::{
    aws::S3,
    models::{
        exif_meta::{ExifMeta, db::Error as ExifMetaDbError},
        exif_meta::{PhotographyDetails, from_exif::PhotographyDetailsError},
        photo::{Error as PhotoError, Photo, db::Error as PhotoDbError},
    },
    sqlx::{SqlitePool, error::Error as SqlxError},
};
use fuji::exif::TryFromExifData;
use itertools::Itertools;
use log::{debug, trace};
use snafu::prelude::*;
use std::{path::Path, sync::mpsc};

#[cfg(target_os = "windows")]
static CAMERA: Emoji<'_, '_> = Emoji("📷", "");
#[cfg(not(target_os = "windows"))]
static CAMERA: Emoji<'_, '_> = Emoji("📷 ", "");
#[cfg(target_os = "windows")]
static FILM: Emoji<'_, '_> = Emoji("🎞️", "");
#[cfg(target_os = "windows")]
static TAG: Emoji<'_, '_> = Emoji("🏷️", "");
#[cfg(not(target_os = "windows"))]
static TAG: Emoji<'_, '_> = Emoji("🏷️  ", "");

pub async fn create(pool: &SqlitePool, src: &Path, s3: &S3) -> Result<(), Error> {
    let mut conn = pool.begin().await.context(TxSnafu)?;

    if Photo::find_by_filename(&mut conn, src)
        .await
        .context(PathPhotoSnafu)?
        .is_some()
    {
        return Err(Error::PhotoExists {
            path: src.to_str().unwrap().to_string(),
        });
    }

    let data = exiftool::spawn::read_metadata(src).context(ExiftoolSnafu)?;
    trace!("Exiftool parsed data: {:?}", data);

    let orientation = orientation::get_orientation(src).context(OrientationSnafu)?;

    let (tx, rx) = mpsc::channel::<ImageProcess>();

    debug!("Building Images to upload");
    let main_handle = start_build(src, tx).context(BuildImagesSnafu)?;

    let title = capture(&format!(
        "{} Please, type the title for the Photograph: ",
        CAMERA
    ));
    trace!("Title: {}", title);
    let tags = capture(&format!(
        "{} Please, type the tags for this photograph: ",
        TAG
    ));
    let tags: Vec<String> = tags
        .split(',')
        .map(|t| t.trim().to_lowercase())
        .unique()
        .collect();
    debug!("Tags: {:?}", tags);

    let recipe = get_some_fujifilm_recipe(&data, &mut conn)
        .await
        .context(FujifilmRecipeSnafu)?;
    debug!("{:?}", recipe);

    let photo = Photo::new(title, src, orientation).context(NewPhotoSnafu)?;
    photo.save(&mut conn).await.context(SavePhotoSnafu)?;
    debug!("{:?}", photo);

    photo
        .save_tags(&mut conn, &tags)
        .await
        .context(AttachTagsSnafu)?;

    let photography_details =
        PhotographyDetails::try_from_exif(data.as_slice()).context(PhotographyDetailsSnafu)?;
    debug!("{:?}", photography_details);

    let exif = ExifMeta::new(photography_details, &photo, &recipe);
    exif.save(&mut conn).await.context(SaveExifSnafu)?;
    debug!("{:?}", exif);

    let buffers = finish_build(rx, main_handle).context(BuildImagesSnafu)?;
    debug!("About to upload to S3");
    upload(&photo, s3, buffers).await.context(UploadSnafu)?;
    debug!("Uploaded to S3");

    conn.commit().await.context(TxSnafu)?;

    Ok(())
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to get EXIF: {}", source))]
    Exiftool { source: exiftool::spawn::Error },

    #[snafu(display("Could not find the PhotographyDetails from EXIF: {}", source))]
    PhotographyDetails { source: PhotographyDetailsError },

    #[snafu(display("Failed to build images: {}", source))]
    BuildImages { source: BuildImagesError },

    #[snafu(display("Failed to execute Transaction: {}", source))]
    Tx { source: SqlxError },

    #[snafu(display("Failed to get Fujifilm Recipe: {}", source))]
    FujifilmRecipe { source: GetFujifilmError },

    #[snafu(display("Failed to check for photo by path: {}", source))]
    PathPhoto { source: PhotoDbError },

    #[snafu(display("Photo with path {} already exists", path))]
    PhotoExists { path: String },

    #[snafu(display("Failed to create a photo object: {}", source))]
    NewPhoto { source: PhotoError },

    #[snafu(display("Failed to upload the images: {}", source))]
    Upload { source: AWSError },

    #[snafu(display("Failed to save the photo: {}", source))]
    SavePhoto { source: PhotoDbError },

    #[snafu(display("Failed to attach tags to the photo: {}", source))]
    AttachTags { source: PhotoDbError },

    #[snafu(display("Failed to save the EXIF data: {}", source))]
    SaveExif { source: ExifMetaDbError },

    #[snafu(display("Failed to get orientation: {}", source))]
    Orientation { source: OrientationError },
}
