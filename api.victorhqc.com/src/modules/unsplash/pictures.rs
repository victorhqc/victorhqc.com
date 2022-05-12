use super::{
    client::{build_client, UnsplashClientError},
    entities::Picture,
};
use snafu::prelude::*;

pub async fn fetch_random_picture() -> Result<Picture> {
    let unsplash_url: &str = dotenv!("UNSPLASH_API_URL");
    let client = build_client().context(ClientIssueSnafu)?;

    let response = client
        .get(format!("{}/photos/random", unsplash_url))
        .send()
        .await
        .context(RequestIssueSnafu)?;

    debug!("Client Response: {:?}", response);

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
