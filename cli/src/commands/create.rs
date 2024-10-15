use crate::{
    exiftool,
    photo::{build_images, upload},
    utils::capture,
};
use core_victorhqc_com::{
    aws::{photo::ImageSize, S3},
    models::{exif_meta::ExifMeta, fujifilm::FujifilmRecipe, photo::Photo},
    sqlx::SqlitePool,
};
use core_victorhqc_com::{
    exif::FromExifData,
    models::{
        exif_meta::{Maker, PhotographyDetails},
        fujifilm::FujifilmRecipeDetails,
    },
};
use log::{debug, trace};
use std::{path::Path, sync::mpsc};

pub async fn create(
    pool: &SqlitePool,
    src: &Path,
    s3: &S3,
) -> Result<(), Box<dyn std::error::Error>> {
    let data =
        exiftool::spawn::read_metadata(src).expect("Failed to get exif metadata from exiftool");
    trace!("Exiftool parsed data: {:?}", data);

    let maker = Maker::from_exif(data.as_slice()).expect("Could not get Maker from exiftool");
    debug!("{:?}", maker);

    let channel = mpsc::channel::<(ImageSize, Vec<u8>)>();

    debug!("Building Images to upload");
    let buffers = build_images(src, channel).expect("Failed to compress images");

    let photography_details = PhotographyDetails::from_exif(data.as_slice())
        .expect("Could not get photography details from exiftool");
    debug!("{:?}", photography_details);

    let title = capture("📷  Please, type the title for the Photograph: ");
    debug!("Title: {}", title);

    let mut tx = pool.begin().await?;

    let mut recipe: Option<FujifilmRecipe> = None;
    if maker == Maker::Fujifilm {
        let recipe_details = FujifilmRecipeDetails::from_exif(data.as_slice())
            .expect("Could not get fujifilm recipe from exiftool");
        debug!("{:?}", recipe_details);

        recipe = FujifilmRecipe::find_by_details(pool, &recipe_details)
            .await
            .expect("Failed to query for existing recipe");

        if recipe.is_none() {
            println!();
            let recipe_name = capture("🎞️  Please, specify the name of the recipe used: ");
            debug!("Recipe Name: {}", recipe_name);

            let r = FujifilmRecipe::new(recipe_name, recipe_details);

            r.save(&mut tx)
                .await
                .expect("Failed to save Fujifilm recipe");

            recipe = Some(r);
        }
    }

    debug!("{:?}", recipe);

    let photo = Photo::new(title, src).unwrap();

    upload(&photo, s3, buffers)
        .await
        .expect("Failed to upload photos");

    debug!("{:?}", photo);

    photo.save(&mut tx).await.expect("Failed to store Photo");

    let exif = ExifMeta::new(photography_details, &photo, &recipe);
    exif.save(&mut tx).await.expect("Failed to save Exif");

    debug!("{:?}", exif);

    tx.commit().await.expect("Failed to commit transaction");

    Ok(())
}
