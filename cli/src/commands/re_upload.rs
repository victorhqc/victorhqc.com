use crate::{
    exiftool,
    photo::{
        aws::{Error as AWSError, remove, upload},
        build_images::{Error as BuildImagesError, ImageProcess, finish_build, start_build},
        orientation::{self, OrientationError},
    },
    utils::{GetFujifilmError, get_some_fujifilm_recipe},
};
use core_victorhqc_com::{
    aws::S3,
    models::{
        exif_meta::{
            ExifMeta, PhotographyDetails, db::Error as ExifMetaDbError, from_exif::TryFromExifData,
        },
        photo::{Error as PhotoError, Photo, db::Error as PhotoDbError},
    },
    sqlx::SqlitePool,
};
use core_victorhqc_com::{
    models::exif_meta::from_exif::PhotographyDetailsError, sqlx::error::Error as SqlxError,
};
use snafu::prelude::*;
use std::{path::Path, sync::mpsc};

pub async fn re_upload(pool: &SqlitePool, id: String, src: &Path, s3: &S3) -> Result<(), Error> {
    let mut conn = pool.begin().await.context(DBConnectSnafu)?;

    let mut photo = Photo::find_by_id(&mut conn, &id)
        .await
        .context(PhotoByIdSnafu)?;

    let data = exiftool::spawn::read_metadata(src).context(ExiftoolSnafu)?;
    trace!("Exiftool parsed data: {:?}", data);

    let orientation = orientation::get_orientation(src).context(OrientationSnafu)?;

    let (tx, rx) = mpsc::channel::<ImageProcess>();

    debug!("Building Images to Re-upload");
    let main_handle = start_build(src, tx).context(BuildImagesSnafu)?;

    photo
        .update_file(src, &orientation)
        .context(UpdatePhotoSnafu)?;
    photo.update(&mut conn).await.context(UpdateDbPhotoSnafu)?;

    let recipe = get_some_fujifilm_recipe(&data, &mut conn)
        .await
        .context(FujifilmRecipeSnafu)?;
    debug!("{:?}", recipe);

    let photography_details =
        PhotographyDetails::try_from_exif(data.as_slice()).context(PhotographyDetailsSnafu)?;
    debug!("{:?}", photography_details);

    let exif = ExifMeta::new(photography_details, &photo, &recipe);
    exif.replace(&mut conn).await.context(SaveExifSnafu)?;
    debug!("{:?}", exif);

    debug!("Removing Photos from AWS");
    remove(&photo, s3).await.context(RemoveSnafu)?;

    let buffers = finish_build(rx, main_handle).context(BuildImagesSnafu)?;

    photo.set_blurhash(buffers.blurhash.clone());
    photo.update(&mut conn).await.context(UpdateDbPhotoSnafu)?;

    debug!("About to upload to S3");
    upload(&photo, s3, buffers).await.context(UploadSnafu)?;
    debug!("Uploaded to S3");

    conn.commit().await.context(TxSnafu)?;

    Ok(())
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to connect to db: {}", source))]
    DBConnect { source: SqlxError },

    #[snafu(display("Failed to update photo: {}", source))]
    UpdatePhoto { source: PhotoError },

    #[snafu(display("Failed to update photo in the db: {}", source))]
    UpdateDbPhoto { source: PhotoDbError },

    #[snafu(display("Failed to execute Transaction: {}", source))]
    Tx { source: SqlxError },

    #[snafu(display("Failed to check for photo by id: {}", source))]
    PhotoById { source: PhotoDbError },

    #[snafu(display("Failed to build images: {}", source))]
    BuildImages { source: BuildImagesError },

    #[snafu(display("Failed to Upload {}", source))]
    Upload { source: AWSError },

    #[snafu(display("Failed to Remove {}", source))]
    Remove { source: AWSError },

    #[snafu(display("Failed to save the EXIF data: {}", source))]
    SaveExif { source: ExifMetaDbError },

    #[snafu(display("Failed to get EXIF: {}", source))]
    Exiftool { source: exiftool::spawn::Error },

    #[snafu(display("Could not find the PhotographyDetails from EXIF: {}", source))]
    PhotographyDetails { source: PhotographyDetailsError },

    #[snafu(display("Failed to get Fujifilm Recipe: {}", source))]
    FujifilmRecipe { source: GetFujifilmError },

    #[snafu(display("Failed to get orientation: {}", source))]
    Orientation { source: OrientationError },
}
