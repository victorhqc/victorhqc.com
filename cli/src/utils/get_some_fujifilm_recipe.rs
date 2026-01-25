#[cfg(not(target_os = "windows"))]
use console::Emoji;
use core_victorhqc_com::{
    models::{
        exif_meta::CameraMaker,
        fujifilm::{FujifilmRecipe, db::Error as FujifilmDbError},
    },
    sqlx::{Sqlite, Transaction},
};
use fuji::{
    exif::{ExifData, FromExifData},
    recipe::{FujifilmRecipe as _FujifilmRecipe, FujifilmRecipeDetails},
};
use snafu::prelude::*;

use crate::utils::capture;

#[cfg(not(target_os = "windows"))]
static FILM: Emoji<'_, '_> = Emoji("🎞️  ", "");

pub async fn get_some_fujifilm_recipe<'a>(
    data: &'a Vec<ExifData>,
    conn: &'a mut Transaction<'_, Sqlite>,
) -> Result<Option<FujifilmRecipe>, GetFujifilmError> {
    let maker = CameraMaker::from_exif(data.as_slice()).context(MakerSnafu)?;
    debug!("{:?}", maker);

    let mut recipe: Option<FujifilmRecipe> = None;
    if maker == CameraMaker::Fujifilm {
        let recipe_details = FujifilmRecipeDetails::from_exif(data.as_slice());
        debug!("{:?}", recipe_details);

        // Recipes are optional, not all the photos will have. Only SoC will.
        if let Some(recipe_details) = recipe_details {
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
    }

    Ok(recipe)
}

#[derive(Debug, Snafu)]
pub enum GetFujifilmError {
    #[snafu(display("Could not get Maker from EXIF"))]
    Maker,

    #[snafu(display("Failed to find recipe: {}", source))]
    FujifilmFindRecipe { source: FujifilmDbError },

    #[snafu(display("Failed to save the recipe: {}", source))]
    FujifilmSaveRecipe { source: FujifilmDbError },
}
