use super::context::render_content;
use crate::{requests, state::AppState};
use actix_web::{error::ResponseError, get, web, HttpResponse, Responder, Result};
use snafu::prelude::*;
use std::str::FromStr;
use strum_macros::{Display, EnumString};
use tera::Context;

#[get("/portfolio")]
pub async fn portfolio(data: web::Data<AppState>) -> Result<impl Responder> {
    let mut context = Context::new();
    let portfolio = requests::photos::get_photos_from_tag("portfolio")
        .await
        .context(PortfolioSnafu)?;

    context.insert("portfolio_photos", &portfolio);

    let content = render_content("portfolio", &mut context, &data)?;

    Ok(HttpResponse::Ok().body(content))
}

#[get("/collection/{name}")]
pub async fn collection(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    let collection_name = path.into_inner();
    let collection_name =
        Collection::from_str(&collection_name).context(UnknownCollectionSnafu {
            name: collection_name,
        })?;

    let mut context = Context::new();

    let collection = requests::photos::get_photos_from_tag(&collection_name.to_string())
        .await
        .context(PortfolioSnafu)?;

    context.insert("portfolio_photos", &collection);

    let content = render_content("_blocks/portfolio_collection", &mut context, &data)?;

    Ok(HttpResponse::Ok().body(content))
}

#[derive(Debug, Display, PartialEq, EnumString)]
enum Collection {
    #[strum(serialize = "portfolio")]
    Portfolio,
    #[strum(serialize = "berlin")]
    Berlin,
    #[strum(serialize = "japan")]
    Japan,
}

#[derive(Debug, Snafu)]
pub enum Error {
    Portfolio {
        source: requests::photos::Error,
    },
    UnknownCollection {
        name: String,
        source: strum::ParseError,
    },
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::Portfolio { source } => {
                HttpResponse::InternalServerError().body(source.to_string())
            }
            Error::UnknownCollection { name, source: _ } => {
                HttpResponse::BadRequest().body(format!("Unknown Collection: {}", name))
            }
        }
    }
}
