use crate::state::AppState;
use actix_files as fs;
use actix_web::{middleware, web, App, HttpServer};
use lazy_static::lazy_static;
use snafu::prelude::*;
use tera::Tera;

mod requests;
mod routes;
mod state;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let root = std::env::var("WEB_TEMPLATES_ROOT").unwrap_or("".to_string());
        let mut tera = match Tera::new(&format!("{}templates/**/*", root)) {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html", ".css", ".js"]);

        tera
    };
}

#[actix_web::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv().unwrap();
    let api_host = std::env::var("WEB_API_HOST").expect("WEB_API_HOST env variable is missing");

    if TEMPLATES.templates.is_empty() {
        return Err(Error::MissingTemplates);
    }

    let photos = requests::photos::get_photos_from_tag("portfolio")
        .await
        .context(PortfolioSnafu)?;

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .app_data(web::Data::new(AppState {
                api_host: api_host.clone(),
                portfolio_photos: photos.clone(),
            }))
            .service(fs::Files::new("/static", "./static"))
            .service(routes::index::index)
    })
    .workers(4)
    .bind(("127.0.0.1", 7879))
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
