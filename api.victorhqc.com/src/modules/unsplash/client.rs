use reqwest::{
    header::{HeaderMap, HeaderName, AUTHORIZATION},
    Client,
};
use snafu::prelude::*;
use std::env;

pub fn build_client() -> Result<Client> {
    let client_id = env::var("UNSPLASH_ACCESS_KEY").expect("UNSPLASH_ACCESS_KEY is not defined");

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        format!("Client-ID {}", client_id).parse().unwrap(),
    );

    headers.insert(
        HeaderName::from_static("accept-version"),
        "v1".parse().unwrap(),
    );

    debug!("Client Headers: {:?}", headers);

    let client = Client::builder()
        .default_headers(headers)
        .build()
        .context(ClientIssueSnafu)?;

    debug!("Client Created OK");

    Ok(client)
}

pub type Result<T, E = UnsplashClientError> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum UnsplashClientError {
    #[snafu(display("Failed build a client {}", source))]
    ClientIssue { source: reqwest::Error },
}
