use crate::{graphql::loaders::AppLoader, graphql::models::ExifMeta as GqlExifMeta};
use async_graphql::{dataloader::Loader, Result};
use core_victorhqc_com::models::exif_meta::{db::Error as DbError, ExifMeta};
use snafu::prelude::*;
use std::{
    cmp::{Eq, PartialEq},
    collections::HashMap,
    hash::{Hash, Hasher},
    sync::Arc,
};

impl Loader<ExifMetaByPhotoId> for AppLoader {
    type Value = GqlExifMeta;
    type Error = Arc<Error>;

    async fn load(
        &self,
        ids: &[ExifMetaByPhotoId],
    ) -> Result<HashMap<ExifMetaByPhotoId, Self::Value>, Self::Error> {
        let ids: Vec<String> = ids.iter().map(|i| i.0.clone()).collect();

        debug!("Loading exif meta with ids: {:?}", ids);
        let values = ExifMeta::find_by_photo_ids(&self.pool, &ids)
            .await
            .context(QuerySnafu)?;

        let mut grouped: HashMap<ExifMetaByPhotoId, Self::Value> = HashMap::new();
        for value in values.into_iter() {
            let id = ExifMetaByPhotoId::new(&value.photo_id);

            grouped.insert(id, value.into());
        }

        Ok(grouped)
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct ExifMetaByPhotoId(String);

impl ExifMetaByPhotoId {
    pub fn new(id: &str) -> Self {
        Self(String::from(id))
    }
}

impl Hash for ExifMetaByPhotoId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("{:?}", source))]
    QueryError { source: DbError },
}
