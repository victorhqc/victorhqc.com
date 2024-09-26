use crate::{
    graphql::loaders::AppLoader,
    graphql::models::ExifMeta as GqlExifMeta,
    models::exif_meta::{db::Error as ExifMetaError, ExifMeta},
};
use async_graphql::{dataloader::Loader, Result};
use snafu::prelude::*;
use std::{
    cmp::{Eq, PartialEq},
    collections::HashMap,
    hash::{Hash, Hasher},
    sync::Arc,
};

impl Loader<ExifMetaId> for AppLoader {
    type Value = GqlExifMeta;
    type Error = Arc<Error>;

    async fn load(
        &self,
        ids: &[ExifMetaId],
    ) -> Result<HashMap<ExifMetaId, Self::Value>, Self::Error> {
        let ids: Vec<String> = ids.into_iter().map(|i| i.0.clone()).collect();

        let values = ExifMeta::find_by_ids(&self.pool, &ids)
            .await
            .context(QuerySnafu)?;

        let mut grouped: HashMap<ExifMetaId, Self::Value> = HashMap::new();
        for value in values.into_iter() {
            let id = ExifMetaId::new(&value.id);

            grouped.insert(id, value.into());
        }

        Ok(grouped)
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct ExifMetaId(String);

impl ExifMetaId {
    pub fn new(id: &str) -> Self {
        Self(String::from(id))
    }
}

impl Hash for ExifMetaId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("{:?}", source))]
    QueryError { source: ExifMetaError },
}
