mod commands;
mod exiftool;
mod photo;
mod utils;

use clap::{Parser, Subcommand};
use core_victorhqc_com::{aws::S3, db::get_pool};
use log::{debug, error};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::current_dir().unwrap();

    dotenvy::from_path(path.join(".env")).unwrap();
    pretty_env_logger::init();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL env variable is missing");
    debug!("db_url {}", db_url);

    let bucket_name =
        std::env::var("AWS_BUCKET_NAME").expect("AWS_BUCKET_NAME env variable is missing");

    let pool = get_pool(&db_url).await.unwrap();
    let s3 = S3::new(&bucket_name).await;

    let args = Cli::parse();

    debug!("CLI: {:?}", args);

    match args.command {
        Commands::Create { source } => {
            let src = Path::new(&source);

            commands::create::create(&pool, src, &s3)
                .await
                .map_err(|e| {
                    error!("Failed to create Image: {}", e);

                    e
                })
                .unwrap();
        }
        Commands::ReUpload { source } => {
            let src = Path::new(&source);

            commands::re_upload::re_upload(&pool, src, &s3)
                .await
                .map_err(|e| {
                    error!("Failed to re-upload Image: {}", e);

                    e
                })
                .unwrap();
        }
        #[cfg(debug_assertions)]
        Commands::DebugCompression { source } => {
            let src = Path::new(&source);

            commands::debug_compression::debug_compression(src)
                .await
                .unwrap();
        }
    }

    Ok(())
}

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Create {
        #[arg(short, long)]
        source: String,
    },
    #[command(arg_required_else_help = true)]
    ReUpload {
        #[arg(short, long)]
        source: String,
    },
    #[cfg(debug_assertions)]
    DebugCompression {
        #[arg(short, long)]
        source: String,
    },
}
