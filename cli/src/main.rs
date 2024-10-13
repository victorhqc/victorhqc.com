mod exiftool;

use clap::Parser;
use core_victorhqc_com::db::get_pool;
use core_victorhqc_com::models::photo::Photo;
use core_victorhqc_com::{
    exif::FromExifData,
    models::{
        exif_meta::{Maker, PhotographyDetails},
        fujifilm::FujifilmRecipeDetails,
    },
};
use log::debug;
use std::io;
use std::io::Write;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::current_dir().unwrap();

    dotenvy::from_path(path.join(".env")).unwrap();
    pretty_env_logger::init();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL env variable is missing");

    let pool = get_pool(&db_url).await.unwrap();

    let args = Args::parse();
    debug!("Arguments: {:?}", args);

    let src = Path::new(&args.source);

    let data =
        exiftool::spawn::read_metadata(src).expect("Failed to get exif metadata from exiftool");
    debug!("Exiftool parsed data: {:?}", data);

    let maker = Maker::from_exif(data.as_slice()).expect("Could not get Maker from exiftool");
    debug!("{:?}", maker);

    let photography_details = PhotographyDetails::from_exif(data.as_slice())
        .expect("Could not get photography details from exiftool");
    debug!("{:?}", photography_details);

    if maker == Maker::Fujifilm {
        let recipe = FujifilmRecipeDetails::from_exif(data.as_slice())
            .expect("Could not get fujifilm recipe from exiftool");
        debug!("{:?}", recipe);
    }

    print!("Please, type the title for the Photograph: ");
    io::stdout().flush().unwrap();

    let mut title = String::new();
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to capture the title of the photograph");

    debug!("Title: {}", title);

    let mut tx = pool.begin().await?;

    let photo = Photo::new(title.trim(), src).unwrap();

    debug!("{:?}", photo);

    photo.save(&mut tx).await.expect("Failed to store Photo");

    tx.commit().await.expect("Failed to commit transaction");

    Ok(())
}

#[derive(Debug, Parser)]
struct Args {
    /// Image Source
    #[clap(short, long)]
    source: String,
}
