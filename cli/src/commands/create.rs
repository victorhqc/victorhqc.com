use crate::{
    exiftool,
    photo::{
        aws::{upload, Error as AWSError},
        build_images::{finish_build, start_build, Error as BuildImagesError, ImageProcess},
    },
    utils::capture,
};
use console::Emoji;
use core_victorhqc_com::{
    aws::S3,
    models::{
        exif_meta::{db::Error as ExifMetaDbError, ExifMeta},
        exif_meta::{CameraMaker, PhotographyDetails},
        fujifilm::{db::Error as FujifilmDbError, FujifilmRecipe},
        photo::{db::Error as PhotoDbError, Error as PhotoError, Photo},
    },
    sqlx::{error::Error as SqlxError, Sqlite, SqlitePool, Transaction},
};
use fuji::{
    exif::{ExifData, FromExifData},
    recipe::{FujifilmRecipe as _FujifilmRecipe, FujifilmRecipeDetails},
};
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
#[cfg(not(target_os = "windows"))]
static FILM: Emoji<'_, '_> = Emoji("🎞️  ", "");
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

    let recipe = get_some_fujifilm_recipe(&data, &mut conn).await?;
    debug!("{:?}", recipe);

    let photo = Photo::new(title, src).context(NewPhotoSnafu)?;
    photo.save(&mut conn).await.context(SavePhotoSnafu)?;
    debug!("{:?}", photo);

    photo
        .save_tags(&mut conn, &tags)
        .await
        .context(AttachTagsSnafu)?;

    let photography_details =
        PhotographyDetails::from_exif(data.as_slice()).context(PhotographyDetailsSnafu)?;
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

async fn get_some_fujifilm_recipe<'a>(
    data: &'a Vec<ExifData>,
    conn: &'a mut Transaction<'_, Sqlite>,
) -> Result<Option<FujifilmRecipe>, Error> {
    let maker = CameraMaker::from_exif(data.as_slice()).context(MakerSnafu)?;
    debug!("{:?}", maker);

    let mut recipe: Option<FujifilmRecipe> = None;
    if maker == CameraMaker::Fujifilm {
        let recipe_details = FujifilmRecipeDetails::from_exif(data.as_slice())
            .context(FujifilmRecipeDetailsSnafu)?;
        debug!("{:?}", recipe_details);

        recipe = FujifilmRecipe::find_by_details(conn, &recipe_details)
            .await
            .context(FujifilmFindRecipeSnafu)?;

        if recipe.is_none() {
            let recipe_name = capture(&format!(
                "{} Please, specify the name of the recipe used: ",
                FILM
            ));
            debug!("Recipe Name: {}", recipe_name);

            let r = FujifilmRecipe::new(recipe_name, _FujifilmRecipe::new(recipe_details));

            r.save(conn).await.context(FujifilmSaveRecipeSnafu)?;

            recipe = Some(r);
        }
    }

    Ok(recipe)
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to get EXIF: {}", source))]
    Exiftool { source: exiftool::spawn::Error },

    #[snafu(display("Could not get Maker from EXIF"))]
    Maker,

    #[snafu(display("Could not find the PhotographyDetails from EXIF"))]
    PhotographyDetails,

    #[snafu(display("Could not find Fujifilm Recipe details from EXIF"))]
    FujifilmRecipeDetails,

    #[snafu(display("Failed to build images: {}", source))]
    BuildImages { source: BuildImagesError },

    #[snafu(display("Failed to execute Transaction: {}", source))]
    Tx { source: SqlxError },

    #[snafu(display("Failed to find recipe: {}", source))]
    FujifilmFindRecipe { source: FujifilmDbError },

    #[snafu(display("Failed to save the recipe: {}", source))]
    FujifilmSaveRecipe { source: FujifilmDbError },

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
}
