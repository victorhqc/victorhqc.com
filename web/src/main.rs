use crate::state::AppState;
use actix_files as fs;
use actix_web::{middleware, web, App, HttpServer};
use lazy_static::lazy_static;
use log::info;
use snafu::prelude::*;
use std::env;
use tera::Tera;

mod gql;
mod requests;
mod routes;
mod state;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let root = std::env::var("WEB_ROOT").unwrap_or("".to_string());
        let mut tera = match Tera::new(&format!("{}templates/**/*", root)) {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html", ".tera", ".css", ".js"]);

        tera
    };
}

#[actix_web::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv().unwrap();
    pretty_env_logger::init();

    let api_host = env::var("WEB_API_HOST").expect("WEB_API_HOST env variable is missing");
    let root = env::var("WEB_ROOT").unwrap_or("".to_string());
    let port = env::var("WEB_PORT").expect("WEB_PORT env variable is missing");
    let port: u16 = port
        .parse::<u16>()
        .expect("WEB_PORT is not a valid integer");

    if TEMPLATES.templates.is_empty() {
        return Err(Error::MissingTemplates);
    }

    let photos = requests::photos::get_photos_from_tag("portfolio")
        .await
        .context(PortfolioSnafu)?;

    info!("Booting Web in port {}", port);

    let static_path = format!("./{}static", root);
    info!("Serving static files from {}", static_path);

    let scripts_path = format!("./{}public", root);
    info!("Serving script files from {}", scripts_path);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .app_data(web::Data::new(AppState {
                api_host: api_host.clone(),
                portfolio_photos: photos.clone(),
            }))
            .service(fs::Files::new("/static", &static_path))
            .service(fs::Files::new("/public", &scripts_path))
            .service(routes::index::index)
            .service(routes::portfolio::portfolio)
            .service(routes::portfolio::portfolio_collection)
            .service(routes::portfolio::collection_photo)
            .service(routes::portfolio::ajax_collection)
            .service(routes::portfolio::ajax_one_photo)
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

    #[snafu(display("Failed to load Porftolio photos: {:?}", source))]
    Portfolio { source: requests::photos::Error },

    #[snafu(display("Failed to load Templates, maybe the path is incorrect"))]
    MissingTemplates,
}
