use reqwest::Error as ReqwestError;
use serde::Deserialize;
use snafu::prelude::*;

#[derive(Deserialize)]
pub struct Photo {
    pub id: String,
    // pub title: String,
}

pub async fn get_photos_from_tag(name: &str) -> Result<Vec<Photo>, Error> {
    let api_host = std::env::var("WEB_API_HOST").expect("WEB_API_HOST env variable is missing");

    let photos: Vec<Photo> = reqwest::get(format!("{}/v1/photos/{}", api_host, name))
        .await
        .context(RequestSnafu)?
        .json()
        .await
        .context(JsonParseSnafu)?;

    Ok(photos)
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Request failed: {:?}", source))]
    Request { source: ReqwestError },

    #[snafu(display("Request Json Deserialization failed: {:?}", source))]
    JsonParse { source: ReqwestError },
}
