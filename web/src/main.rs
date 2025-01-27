use crate::state::AppState;
use actix_files as fs;
use actix_web::{middleware, web, App, HttpServer};
use analytics::{session::Session, visit::Visit};
use lazy_static::lazy_static;
use log::info;
use snafu::prelude::*;
use sqlx::SqlitePool;
use std::{env, path::PathBuf};
use tera::Tera;
use tokio::sync::mpsc;
use uaparser::UserAgentParser;

mod analytics;
mod collections;
mod gql;
mod prefetch;
mod requests;
mod routes;
mod state;
mod tera_utils;
mod utils;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let root = env::var("WEB_ROOT").unwrap_or("".to_string());
        let mut tera = match Tera::new(&format!("{}templates/**/*", root)) {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html", ".tera", ".css", ".js"]);

        tera.register_function(
            "get_film_simulation_img",
            tera_utils::functions::get_film_simulation_image(),
        );
        tera.register_function(
            "parse_film_simulation_name",
            tera_utils::functions::parse_film_simulation_name(),
        );
        tera.register_function("uuid", tera_utils::functions::uuid());
        tera.register_function("gravatar", tera_utils::functions::get_gravatar());

        tera
    };
}

#[actix_web::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv().ok();
    pretty_env_logger::init();

    let api_host = env::var("WEB_API_HOST").expect("WEB_API_HOST env variable is missing");
    let root = env::var("WEB_ROOT").unwrap_or("".to_string());
    let port = env::var("WEB_PORT").expect("WEB_PORT env variable is missing");
    let port: u16 = port
        .parse::<u16>()
        .expect("WEB_PORT is not a valid integer");
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL env variable is missing");

    let regexes_path = PathBuf::from(&root).join("regexes.yaml");

    let file_exists = std::fs::exists(&regexes_path).context(ReadPathSnafu)?;
    if !file_exists {
        return Err(Error::MissingRegex {
            path: regexes_path.as_os_str().to_str().unwrap().to_string(),
        });
    }

    let parser = UserAgentParser::builder()
        .with_unicode_support(false)
        .with_device(true)
        .with_os(true)
        .with_user_agent(false)
        .build_from_yaml(regexes_path.as_os_str().to_str().unwrap())
        .expect("Parser creation failed");

    let pool = SqlitePool::connect(&db_url).await.context(DBSnafu)?;

    if TEMPLATES.templates.is_empty() {
        return Err(Error::MissingTemplates);
    }

    let prefetched = prefetch::fetch_photos().await.context(PrefetchSnafu)?;

    info!("Booting Web in port {}", port);

    let static_path = format!("./{}static", root);
    info!("Serving static files from {}", static_path);

    let scripts_path = format!("./{}public", root);
    info!("Serving public files from {}", scripts_path);

    let (tx, rx) = mpsc::channel::<(Session, Visit)>(100);

    let state = AppState {
        api_host,
        prefetched,
        ua_parser: parser,
        analytics_sender: tx,
    };

    tokio::spawn(async move { analytics::recorder::store(pool, rx).await });

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .app_data(web::Data::new(state.clone()))
            .service(
                fs::Files::new("/static", &static_path)
                    .use_etag(true)
                    .use_last_modified(true),
            )
            .service(
                fs::Files::new("/public", &scripts_path)
                    .use_etag(true)
                    .use_last_modified(true),
            )
            .service(routes::index::index)
            .service(routes::portfolio::photography)
            .service(routes::portfolio::portfolio_collection)
            .service(routes::portfolio::collection_photo)
            .service(routes::portfolio::ajax_collection)
            .service(routes::portfolio::ajax_one_photo)
            .service(analytics::routes::register_visit)
    })
    .workers(4)
    .bind(("127.0.0.1", port))
    .context(BindSnafu)?
    .run()
    .await
    .context(StartSnafu)
}

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Failed to bind the server: {:?}", source))]
    Bind { source: std::io::Error },

    #[snafu(display("Failed to start server: {:?}", source))]
    Start { source: std::io::Error },

    #[snafu(display("Failed to load Templates, maybe the path is incorrect"))]
    MissingTemplates,

    #[snafu(display("regexes.yaml file not found at {}", path))]
    MissingRegex { path: String },

    #[snafu(display("Can't read path: {}", source))]
    ReadPath { source: std::io::Error },

    #[snafu(display("Failed to prefetch photos: {:?}", source))]
    Prefetch { source: prefetch::Error },

    #[snafu(display("Failed to connect to analytics db: {:?}", source))]
    DB { source: sqlx::Error },
}
