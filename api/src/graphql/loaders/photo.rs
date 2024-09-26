use crate::{
    graphql::loaders::AppLoader,
    graphql::models::Photo as GqlPhoto,
    models::photo::{db::Error as DbError, Photo},
};
use async_graphql::{dataloader::Loader, Result};
use snafu::prelude::*;
use std::{
    cmp::{Eq, PartialEq},
    collections::HashMap,
    hash::{Hash, Hasher},
    sync::Arc,
};

impl Loader<TagPhotoId> for AppLoader {
    type Value = Vec<GqlPhoto>;
    type Error = Arc<Error>;

    async fn load(
        &self,
        ids: &[TagPhotoId],
    ) -> Result<HashMap<TagPhotoId, Self::Value>, Self::Error> {
        let ids: Vec<String> = ids.into_iter().map(|i| i.0.clone()).collect();

        let values = Photo::find_by_tag_ids(&self.pool, &ids)
            .await
            .context(QuerySnafu)?;

        let mut grouped: HashMap<TagPhotoId, Self::Value> = HashMap::new();

        for (tag_id, photo) in values.into_iter() {
            let id = TagPhotoId::new(&tag_id);
            let gql: GqlPhoto = photo.into();

            if grouped.contains_key(&id) {
                grouped.entry(id).and_modify(|p | p.push(gql));
            } else {
                grouped.insert(id, vec![gql]);
            }
        }

        Ok(grouped)
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct TagPhotoId(String);

impl TagPhotoId {
    pub fn new(id: &str) -> Self {
        Self(String::from(id))
    }
}

impl Hash for TagPhotoId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("{:?}", source))]
    QueryError { source: DbError },
}

