use crate::{
    graphql::loaders::AppLoader,
    graphql::models::Photo as GqlPhoto,
    models::photo::{db::Error as PhotoError, Photo},
};
use async_graphql::{dataloader::Loader, Result};
use snafu::prelude::*;
use std::{
    cmp::{Eq, PartialEq},
    collections::HashMap,
    hash::{Hash, Hasher},
    sync::Arc,
};

impl Loader<PhotoId> for AppLoader {
    type Value = GqlPhoto;
    type Error = Arc<Error>;

    async fn load(
        &self,
        photo_ids: &[PhotoId],
    ) -> Result<HashMap<PhotoId, Self::Value>, Self::Error> {
        let ids: Vec<String> = photo_ids.into_iter().map(|i| i.0.clone()).collect();

        let photos = Photo::find_by_ids(&self.pool, &ids)
            .await
            .context(QuerySnafu)?;

        let mut grouped: HashMap<PhotoId, Self::Value> = HashMap::new();
        for photo in photos.into_iter() {
            let id = PhotoId::new(&photo.id);

            grouped.insert(id, photo.into());
        }

        Ok(grouped)
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct PhotoId(String);

impl PhotoId {
    pub fn new(id: &str) -> Self {
        Self(String::from(id))
    }
}

impl Hash for PhotoId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("{:?}", source))]
    QueryError { source: PhotoError },
}
