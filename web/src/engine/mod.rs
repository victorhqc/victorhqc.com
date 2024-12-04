use axum::{
    async_trait,
    extract::{rejection::MatchedPathRejection, FromRequestParts, MatchedPath},
    http::request::Parts,
    RequestPartsExt,
};
use axum_template::engine::Engine;
use tera::Tera;

pub type AppEngine = Engine<Tera>;

// Because Tera::new loads an entire folder, we need to remove the `/` prefix
// and add a `.html` suffix. We can implement our own custom key extractor that
// transform the key
pub struct CustomKey(pub String);

#[async_trait]
impl<S> FromRequestParts<S> for CustomKey
where
    S: Send + Sync,
{
    type Rejection = MatchedPathRejection;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let key: Vec<char> = parts
            // `axum_template::Key` internally uses `axum::extract::MatchedPath`
            .extract::<MatchedPath>()
            .await?
            .as_str()
            // Cargo doesn't allow `:` as a file name
            .replace(":", "$")
            .chars()
            // Remove the first character `/`
            .skip(1)
            // Add the `.html` suffix
            .chain(".html".chars())
            .collect();

        let key: String = key.into_iter().collect();

        let key = match key.as_str() {
            ".html" => "index.html".to_string(),
            _ => key,
        };

        Ok(CustomKey(key))
    }
}
