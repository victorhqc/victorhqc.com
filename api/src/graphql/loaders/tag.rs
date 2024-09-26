use crate::{
    graphql::loaders::AppLoader,
    graphql::models::Tag as GqlTag,
    models::tag::{db::Error as DbError, Tag},
};
use async_graphql::{dataloader::Loader, Result};
use snafu::prelude::*;
use std::{
    cmp::{Eq, PartialEq},
    collections::HashMap,
    hash::{Hash, Hasher},
    sync::Arc,
};

impl Loader<TagId> for AppLoader {
    type Value = GqlTag;
    type Error = Arc<Error>;

    async fn load(&self, ids: &[TagId]) -> Result<HashMap<TagId, Self::Value>, Self::Error> {
        let ids: Vec<String> = ids.into_iter().map(|i| i.0.clone()).collect();

        let values = Tag::find_by_ids(&self.pool, &ids)
            .await
            .context(QuerySnafu)?;

        let mut grouped: HashMap<TagId, Self::Value> = HashMap::new();
        for value in values.into_iter() {
            let id = TagId::new(&value.id);

            grouped.insert(id, value.into());
        }

        Ok(grouped)
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct TagId(String);

impl TagId {
    pub fn new(id: &str) -> Self {
        Self(String::from(id))
    }
}

impl Hash for TagId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Loader<PhotoTagId> for AppLoader {
    type Value = Vec<GqlTag>;
    type Error = Arc<Error>;

    async fn load(
        &self,
        ids: &[PhotoTagId],
    ) -> Result<HashMap<PhotoTagId, Self::Value>, Self::Error> {
        let ids: Vec<String> = ids.into_iter().map(|i| i.0.clone()).collect();

        let values = Tag::find_by_photo_ids(&self.pool, &ids)
            .await
            .context(QuerySnafu)?;

        let mut grouped: HashMap<PhotoTagId, Self::Value> = HashMap::new();

        for (photo_id, tag) in values.into_iter() {
            let id = PhotoTagId::new(&photo_id);
            let gql_tag: GqlTag = tag.into();

            if grouped.contains_key(&id) {
                grouped.entry(id).and_modify(|d| d.push(gql_tag));
            } else {
                grouped.insert(id, vec![gql_tag]);
            }
        }

        Ok(grouped)
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct PhotoTagId(String);

impl PhotoTagId {
    pub fn new(id: &str) -> Self {
        Self(String::from(id))
    }
}

impl Hash for PhotoTagId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("{:?}", source))]
    QueryError { source: DbError },
}
