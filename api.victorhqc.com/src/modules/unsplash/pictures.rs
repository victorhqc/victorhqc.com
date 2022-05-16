use super::{
    client::{build_client, UnsplashClientError},
    entities::Picture,
};
use snafu::prelude::*;

pub async fn fetch_random_picture(query: &str, orientation: &str) -> Result<Picture> {
    let unsplash_url: &str =
        option_env!("UNSPLASH_API_URL").expect("UNSPLASH_API_URL is not defined");
    let client = build_client().context(ClientIssueSnafu)?;

    let response = client
        .get(format!(
            "{}/photos/random?query={}&orientation={}",
            unsplash_url, query, orientation
        ))
        .send()
        .await
        .context(RequestIssueSnafu)?;

    debug!("Random Picture Response: {:?}", response);

    let response_headers = response.headers();

    debug!("Random Picture Headers: {:?}", response_headers);

    let picture = response.json::<Picture>().await.context(JsonIssueSnafu)?;

    Ok(picture)
}

pub type Result<T, E = UnsplashPicturesError> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum UnsplashPicturesError {
    #[snafu(display("Failed fetch a random picture {}", source))]
    RequestIssue { source: reqwest::Error },

    #[snafu(display("Failed parse a randon picture {}", source))]
    JsonIssue { source: reqwest::Error },

    #[snafu(display("Failed to build the client {}", source))]
    ClientIssue { source: UnsplashClientError },
}
