use super::context::render_content;
use crate::{gql::get_portfolio::GetPortfolioPhotos, requests, state::AppState};
use actix_web::{error::ResponseError, get, web, HttpResponse, Responder, Result};
use snafu::prelude::*;
use std::str::FromStr;
use strum_macros::{Display, EnumString};
use tera::Context;

#[derive(Debug, Clone, Display, PartialEq, EnumString, serde::Serialize)]
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
    name: Collection,
    path: String,
    ajax_path: String,
}

static COLLECTIONS: &[Collection] = &[Collection::Portfolio, Collection::Berlin, Collection::Japan];

#[get("/portfolio")]
pub async fn portfolio(data: web::Data<AppState>) -> Result<impl Responder> {
    let active_collection = Collection::Portfolio;
    let mut context = Context::new();

    let portfolio = get_collection(&active_collection).await?;

    context.insert("portfolio_photos", &portfolio);
    context.insert(
        "collection_route",
        &CollectionRoute::from(&active_collection),
    );
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

    let active_collection =
        Collection::from_str(&collection_name).context(UnknownCollectionSnafu {
            name: collection_name,
        })?;

    let photos = get_collection(&active_collection).await?;

    context.insert("portfolio_photos", &photos);
    context.insert(
        "collection_route",
        &CollectionRoute::from(&active_collection),
    );
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

    let active_collection =
        Collection::from_str(&collection_name).context(UnknownCollectionSnafu {
            name: collection_name,
        })?;

    let collection = get_collection(&active_collection).await?;

    context.insert("portfolio_photos", &collection);

    let content = render_content("_blocks/portfolio_collection", &mut context, &data)?;

    Ok(HttpResponse::Ok().body(content))
}

#[get("/portfolio/{name}/{id}")]
pub async fn collection_photo(
    path: web::Path<(String, String)>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    let (name, id) = path.into_inner();
    let mut context = Context::new();

    let active_collection = Collection::from_str(&name).context(UnknownCollectionSnafu { name })?;

    let photo = requests::one_photo::get_one_photo(id)
        .await
        .context(OnePhotoSnafu)?;

    context.insert(
        "collection_route",
        &CollectionRoute::from(&active_collection),
    );
    context.insert("available_collections", &build_collection_routes());
    context.insert("photo", &photo);

    let content = render_content("photo", &mut context, &data)?;

    Ok(HttpResponse::Ok().body(content))
}

async fn get_collection(value: &Collection) -> std::result::Result<Vec<GetPortfolioPhotos>, Error> {
    let photos = requests::photos::get_photos_from_tag(&value.to_string())
        .await
        .context(PortfolioSnafu)?;

    Ok(photos)
}

fn build_collection_routes() -> Vec<CollectionRoute> {
    COLLECTIONS.iter().map(CollectionRoute::from).collect()
}

impl From<&Collection> for CollectionRoute {
    fn from(value: &Collection) -> Self {
        match value {
            Collection::Portfolio => CollectionRoute {
                path: "/portfolio".to_string(),
                ajax_path: format!("/collection/{}", value),
                name: value.clone(),
            },
            _ => CollectionRoute {
                path: format!("/portfolio/{}", value),
                ajax_path: format!("/collection/{}", value),
                name: value.clone(),
            },
        }
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    Portfolio {
        source: requests::photos::Error,
    },
    OnePhoto {
        source: requests::one_photo::Error,
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
            Error::OnePhoto { source } => {
                HttpResponse::InternalServerError().body(source.to_string())
            }
            Error::UnknownCollection { name, source: _ } => {
                HttpResponse::BadRequest().body(format!("Unknown Collection: {}", name))
            }
        }
    }
}
