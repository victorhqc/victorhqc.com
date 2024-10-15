use crate::{
    exiftool,
    photo::{
        build_images::{finish_build, start_build, Error as BuildImagesError},
        upload::{upload, Error as UploadError},
    },
    utils::capture,
};
use core_victorhqc_com::{
    aws::{photo::ImageSize, S3},
    models::{
        exif_meta::{db::Error as ExifMetaDbError, ExifMeta},
        fujifilm::{db::Error as FujifilmDbError, FujifilmRecipe},
        photo::{db::Error as PhotoDbError, Error as PhotoError, Photo},
    },
    sqlx::SqlitePool,
};
use core_victorhqc_com::{
    exif::FromExifData,
    models::{
        exif_meta::{Maker, PhotographyDetails},
        fujifilm::FujifilmRecipeDetails,
    },
    sqlx::error::Error as SqlxError,
};
use log::{debug, trace};
use snafu::prelude::*;
use std::{path::Path, sync::mpsc};

pub async fn create(pool: &SqlitePool, src: &Path, s3: &S3) -> Result<(), Error> {
    let data = exiftool::spawn::read_metadata(src).context(ExiftoolSnafu)?;
    trace!("Exiftool parsed data: {:?}", data);

    let maker = Maker::from_exif(data.as_slice()).context(MakerSnafu)?;
    debug!("{:?}", maker);

    let photography_details =
        PhotographyDetails::from_exif(data.as_slice()).context(PhotographyDetailsSnafu)?;
    debug!("{:?}", photography_details);

    let (tx, rx) = mpsc::channel::<(ImageSize, Vec<u8>)>();

    debug!("Building Images to upload");
    let main_handle = start_build(src, tx).context(BuildImagesSnafu)?;

    let title = capture("📷  Please, type the title for the Photograph: ");
    debug!("Title: {}", title);

    let mut tx = pool.begin().await.context(TxSnafu)?;

    let mut recipe: Option<FujifilmRecipe> = None;
    if maker == Maker::Fujifilm {
        let recipe_details = FujifilmRecipeDetails::from_exif(data.as_slice())
            .context(FujifilmRecipeDetailsSnafu)?;
        debug!("{:?}", recipe_details);

        recipe = FujifilmRecipe::find_by_details(pool, &recipe_details)
            .await
            .context(FujifilmFindRecipeSnafu)?;

        if recipe.is_none() {
            println!();
            let recipe_name = capture("🎞️  Please, specify the name of the recipe used: ");
            debug!("Recipe Name: {}", recipe_name);

            let r = FujifilmRecipe::new(recipe_name, recipe_details);

            r.save(&mut tx).await.context(FujifilmSaveRecipeSnafu)?;

            recipe = Some(r);
        }
    }

    debug!("{:?}", recipe);

    let photo = Photo::new(title, src).context(NewPhotoSnafu)?;

    photo.save(&mut tx).await.context(SavePhotoSnafu)?;
    debug!("{:?}", photo);

    let exif = ExifMeta::new(photography_details, &photo, &recipe);
    exif.save(&mut tx).await.context(SaveExifSnafu)?;
    debug!("{:?}", exif);

    let buffers = finish_build(rx, main_handle).context(BuildImagesSnafu)?;
    debug!("About to upload to S3");
    upload(&photo, s3, buffers).await.context(UploadSnafu)?;
    debug!("Uploaded to S3");

    tx.commit().await.context(TxSnafu)?;

    Ok(())
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

    #[snafu(display("Failed to create a photo object: {}", source))]
    NewPhoto { source: PhotoError },

    #[snafu(display("Failed to upload the images: {}", source))]
    Upload { source: UploadError },

    #[snafu(display("Failed to save the photo: {}", source))]
    SavePhoto { source: PhotoDbError },

    #[snafu(display("Failed to save the EXIF data: {}", source))]
    SaveExif { source: ExifMetaDbError },
}
