mod exiftool;

use clap::Parser;
use dirs::home_dir;
use log::debug;
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
}

#[derive(Debug, Parser)]
struct Args {
    /// Image Source
    #[clap(
        short, long, default_value_t = home_dir().unwrap().into_os_string().into_string().unwrap()
    )]
    source: String,
}
