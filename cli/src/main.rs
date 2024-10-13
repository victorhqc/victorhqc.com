mod exiftool;

use clap::Parser;
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

fn main() {
    let path = std::env::current_dir().unwrap();

    dotenvy::from_path(path.join(".env")).unwrap();

    pretty_env_logger::init();

    let args = Args::parse();
    debug!("Arguments: {:?}", args);

    let src = Path::new(&args.source);

    let data = exiftool::spawn::read_metadata(src).unwrap();
    debug!("Exiftool parsed data: {:?}", data);

    let maker = Maker::from_exif(data.as_slice()).unwrap();
    debug!("{:?}", maker);

    let photography_details = PhotographyDetails::from_exif(data.as_slice()).unwrap();
    debug!("{:?}", photography_details);

    if maker == Maker::Fujifilm {
        let recipe = FujifilmRecipeDetails::from_exif(data.as_slice()).unwrap();
        debug!("{:?}", recipe);
    }

    print!("Please, type the title for the Photograph: ");
    io::stdout().flush().unwrap();

    let mut title = String::new();
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to capture the title of the photograph");

    debug!("Title: {}", title);

    let photo = Photo::new(title.trim(), src).unwrap();

    debug!("{:?}", photo);
}

#[derive(Debug, Parser)]
struct Args {
    /// Image Source
    #[clap(short, long)]
    source: String,
}
