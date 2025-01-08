use super::context::render_content;
use crate::{gql::get_portfolio::GetPortfolioPhotos, requests, state::AppState};
use actix_web::{error::ResponseError, get, web, HttpResponse, Responder, Result};
use snafu::prelude::*;
use std::str::FromStr;
use strum_macros::{Display, EnumString};
use tera::Context;

#[derive(Debug, Display, PartialEq, EnumString, serde::Serialize)]
enum Collection {
    #[strum(serialize = "portfolio")]
    #[serde(rename(serialize = "portfolio"))]
    Portfolio,
    #[strum(serialize = "berlin")]
    #[serde(rename(serialize = "berlin"))]
    Berlin,
    #[strum(serialize = "japan")]
    #[serde(rename(serialize = "japan"))]
    Japan,
}

#[derive(Debug, serde::Serialize)]
struct CollectionRoute {
    name: &'static Collection,
    path: String,
    ajax_path: String,
}

static COLLECTIONS: &[Collection] = &[Collection::Portfolio, Collection::Berlin, Collection::Japan];

#[get("/portfolio")]
pub async fn portfolio(data: web::Data<AppState>) -> Result<impl Responder> {
    let collection_name = Collection::Portfolio.to_string();
    let mut context = Context::new();

    let portfolio = get_collection(&collection_name).await?;

    context.insert("portfolio_photos", &portfolio);
    context.insert("collection_name", &collection_name);
    context.insert("available_collections", &build_collection_routes());

    let content = render_content("portfolio", &mut context, &data)?;

    Ok(HttpResponse::Ok().body(content))
}

#[get("/portfolio/{name}")]
pub async fn portfolio_collection(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    let collection_name = path.into_inner();
    let mut context = Context::new();

    let photos = get_collection(&collection_name).await?;

    context.insert("portfolio_photos", &photos);
    context.insert("collection_name", &collection_name);
    context.insert("available_collections", &build_collection_routes());

    let content = render_content("portfolio", &mut context, &data)?;

    Ok(HttpResponse::Ok().body(content))
}

#[get("/collection/{name}")]
pub async fn collection(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    let collection_name = path.into_inner();
    let mut context = Context::new();

    let collection = get_collection(&collection_name).await?;

    context.insert("portfolio_photos", &collection);

    let content = render_content("_blocks/portfolio_collection", &mut context, &data)?;

    Ok(HttpResponse::Ok().body(content))
}

async fn get_collection(value: &str) -> std::result::Result<Vec<GetPortfolioPhotos>, Error> {
    let collection_name =
        Collection::from_str(value).context(UnknownCollectionSnafu { name: value })?;

    let photos = requests::photos::get_photos_from_tag(&collection_name.to_string())
        .await
        .context(PortfolioSnafu)?;

    Ok(photos)
}

fn build_collection_routes() -> Vec<CollectionRoute> {
    COLLECTIONS
        .iter()
        .map(|c| {
            let path = if *c == Collection::Portfolio {
                String::from("/portfolio")
            } else {
                format!("/portfolio/{}", c)
            };

            let ajax_path = format!("/collection/{}", c);

            CollectionRoute {
                name: c,
                path,
                ajax_path,
            }
        })
        .collect()
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
