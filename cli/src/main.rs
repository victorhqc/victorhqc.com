mod commands;
mod exiftool;
mod photo;
mod utils;

use clap::Parser;
use core_victorhqc_com::{aws::S3, db::get_pool};
use log::debug;
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

    commands::create(&pool, src, &s3)
        .await
        .expect("Failed to create Image");

    Ok(())
}

#[derive(Debug, Parser)]
struct Args {
    /// Image Source
    #[clap(short, long)]
    source: String,
}
