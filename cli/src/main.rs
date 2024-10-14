mod exiftool;
mod photo;
mod utils;

use crate::photo::images_to_upload;
use clap::Parser;
use core_victorhqc_com::aws::{photo::ImageSize, S3};
use core_victorhqc_com::db::get_pool;
use core_victorhqc_com::models::exif_meta::ExifMeta;
use core_victorhqc_com::models::fujifilm::FujifilmRecipe;
use core_victorhqc_com::models::photo::Photo;
use core_victorhqc_com::sqlx::SqlitePool;
use core_victorhqc_com::{
    exif::FromExifData,
    models::{
        exif_meta::{Maker, PhotographyDetails},
        fujifilm::FujifilmRecipeDetails,
    },
};
use log::{debug, trace};
use std::io;
use std::io::Write;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::current_dir().unwrap();

    dotenvy::from_path(path.join(".env")).unwrap();
    pretty_env_logger::init();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL env variable is missing");
    let bucket_name =
        std::env::var("AWS_BUCKET_NAME").expect("AWS_BUCKET_NAME env variable is missing");

    let pool = get_pool(&db_url).await.unwrap();

    let args = Args::parse();
    debug!("Arguments: {:?}", args);

    let src = Path::new(&args.source);
    let s3 = S3::new(&bucket_name).await;

    create(&pool, src, &s3).await?;

    Ok(())
}

async fn create(pool: &SqlitePool, src: &Path, s3: &S3) -> Result<(), Box<dyn std::error::Error>> {
    let data =
        exiftool::spawn::read_metadata(src).expect("Failed to get exif metadata from exiftool");
    trace!("Exiftool parsed data: {:?}", data);

    let maker = Maker::from_exif(data.as_slice()).expect("Could not get Maker from exiftool");
    debug!("{:?}", maker);

    debug!("Building Images to upload");
    let buffers = images_to_upload(src)
        .await
        .expect("Failed to compress images");

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

    let photo = Photo::new(title.trim(), src).unwrap();

    s3.upload_to_aws_s3((&photo, ImageSize::Hd), buffers.hd)
        .await
        .expect("Failed to upload HD Photo");

    s3.upload_to_aws_s3((&photo, ImageSize::Md), buffers.md)
        .await
        .expect("Failed to upload MD Photo");

    s3.upload_to_aws_s3((&photo, ImageSize::Sm), buffers.sm)
        .await
        .expect("Failed to upload SM Photo");

    debug!("{:?}", photo);

    photo.save(&mut tx).await.expect("Failed to store Photo");

    let exif = ExifMeta::new(photography_details, &photo, &recipe);
    exif.save(&mut tx).await.expect("Failed to save Exif");

    debug!("{:?}", exif);

    tx.commit().await.expect("Failed to commit transaction");

    Ok(())
}

fn capture(msg: &str) -> String {
    let mut capture = String::new();
    print!("{}", msg);
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut capture)
        .expect("Failed to capture String");

    capture
}

#[derive(Debug, Parser)]
struct Args {
    /// Image Source
    #[clap(short, long)]
    source: String,
}
