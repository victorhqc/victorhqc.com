mod exiftool;

use clap::Parser;
use core_victorhqc_com::{
    exif::FromExifData,
    models::{exif_meta::Maker, fujifilm::FujifilmRecipe},
};
use log::debug;
use std::path::Path;
use core_victorhqc_com::models::exif_meta::ExifMeta;

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
    
    let exif_meta = ExifMeta::from_exif(data.as_slice()).unwrap();
    debug!("{:?}", exif_meta);

    if maker == Maker::Fujifilm {
        let recipe = FujifilmRecipe::from_exif(data.as_slice()).unwrap();
        debug!("Fujifilm Recipe: {:?}", recipe);
    }
}

#[derive(Debug, Parser)]
struct Args {
    /// Image Source
    #[clap(short, long)]
    source: String,
}
