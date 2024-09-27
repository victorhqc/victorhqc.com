use crate::{
    graphql::loaders::AppLoader,
    graphql::models::ExifMeta as GqlExifMeta,
    models::exif_meta::{db::Error as DbError, ExifMeta},
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
    type Value = GqlExifMeta;
    type Error = Arc<Error>;

    async fn load(
        &self,
        ids: &[PhotoId],
    ) -> Result<HashMap<PhotoId, Self::Value>, Self::Error> {
        let ids: Vec<String> = ids.iter().map(|i| i.0.clone()).collect();

        debug!("Loading exif meta with ids: {:?}", ids);
        let values = ExifMeta::find_by_photo_ids(&self.pool, &ids)
            .await
            .context(QuerySnafu)?;

        let mut grouped: HashMap<PhotoId, Self::Value> = HashMap::new();
        for value in values.into_iter() {
            let id = PhotoId::new(&value.photo_id);

            grouped.insert(id, value.into());
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
    QueryError { source: DbError },
}
