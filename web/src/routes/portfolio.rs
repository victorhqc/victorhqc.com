use super::context::{render_content, TemplateKind};
use crate::{gql::get_portfolio::GetPortfolioPhotos, requests, state::AppState};
use actix_web::{error::ResponseError, get, web, HttpResponse, Responder, Result};
use snafu::prelude::*;
use std::str::FromStr;
use strum_macros::{Display, EnumString};
use tera::Context;

#[derive(Debug, serde::Serialize)]
struct PortfolioPhoto {
    photo: GetPortfolioPhotos,
    next_id: String,
    prev_id: String,
    len: usize,
    index: usize,
}

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

    let content = render_content("portfolio", TemplateKind::Html, &mut context, &data)?;

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

    let content = render_content("portfolio", TemplateKind::Html, &mut context, &data)?;

    Ok(HttpResponse::Ok().body(content))
}

#[get("/collection/{name}")]
pub async fn ajax_collection(
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

    context.insert(
        "collection_route",
        &CollectionRoute::from(&active_collection),
    );
    context.insert("portfolio_photos", &collection);

    let content = render_content(
        "_ajax/portfolio_collection",
        TemplateKind::Html,
        &mut context,
        &data,
    )?;

    Ok(HttpResponse::Ok().body(content))
}

#[get("/one_photo/{name}/{id}")]
pub async fn ajax_one_photo(
    path: web::Path<(String, String)>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    let (name, id) = path.into_inner();
    let mut context = Context::new();

    let active_collection = Collection::from_str(&name).context(UnknownCollectionSnafu { name })?;

    let collection = get_collection(&active_collection).await?;
    let photo = collection.iter().find(|p| p.photo.id == id).unwrap();

    context.insert(
        "collection_route",
        &CollectionRoute::from(&active_collection),
    );
    context.insert("photo", &photo);

    let content = render_content("_ajax/one_photo", TemplateKind::Html, &mut context, &data)?;

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

    let collection = get_collection(&active_collection).await?;
    let photo = collection.iter().find(|p| p.photo.id == id).unwrap();

    context.insert(
        "collection_route",
        &CollectionRoute::from(&active_collection),
    );
    context.insert("available_collections", &build_collection_routes());
    context.insert("photo", &photo);

    let content = render_content("photo", TemplateKind::Html, &mut context, &data)?;

    Ok(HttpResponse::Ok().body(content))
}

async fn get_collection(value: &Collection) -> std::result::Result<Vec<PortfolioPhoto>, Error> {
    let photos = requests::photos::get_photos_from_tag(&value.to_string())
        .await
        .context(PortfolioSnafu)?;

    let len = photos.len();

    let first_id = photos.first().map(|p| p.id.clone());
    let mut iter = photos.iter().enumerate().peekable();

    let mut result: Vec<PortfolioPhoto> = Vec::new();
    while let Some((index, photo)) = iter.next() {
        let next_id = if let Some((_, p)) = iter.peek() {
            p.id.clone()
        } else {
            first_id.as_ref().unwrap().clone()
        };

        let prev_index: usize = if index == 0 {
            photos.len() - 1
        } else {
            index - 1
        };
        let prev_id = photos.get(prev_index).map(|p| p.id.clone()).unwrap();

        result.push(PortfolioPhoto {
            photo: photo.clone(),
            next_id,
            prev_id,
            index,
            len,
        });
    }

    Ok(result)
}

fn build_collection_routes() -> Vec<CollectionRoute> {
    COLLECTIONS.iter().map(CollectionRoute::from).collect()
}

impl From<&Collection> for CollectionRoute {
    fn from(value: &Collection) -> Self {
        let ajax_path = format!("/collection/{}", value);

        match value {
            Collection::Portfolio => CollectionRoute {
                path: "/portfolio".to_string(),
                name: value.clone(),
                ajax_path,
            },
            _ => CollectionRoute {
                path: format!("/portfolio/{}", value),
                name: value.clone(),
                ajax_path,
            },
        }
    }
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
