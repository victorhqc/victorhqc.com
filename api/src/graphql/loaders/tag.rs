use crate::{graphql::loaders::AppLoader, graphql::models::Tag as GqlTag};
use async_graphql::{dataloader::Loader, Result};
use core_victorhqc_com::{
    models::tag::{db::Error as DbError, Tag},
    sqlx::Error as SqlxError,
    utils::hashmap::InsertOrPush,
};
use snafu::prelude::*;
use std::{
    cmp::{Eq, PartialEq},
    collections::HashMap,
    hash::{Hash, Hasher},
    sync::Arc,
};

impl Loader<TagById> for AppLoader {
    type Value = GqlTag;
    type Error = Arc<Error>;

    async fn load(&self, ids: &[TagById]) -> Result<HashMap<TagById, Self::Value>, Self::Error> {
        let ids: Vec<String> = ids.iter().map(|i| i.0.clone()).collect();

        let mut conn = self.pool.acquire().await.context(ConnectionSnafu)?;

        let values = Tag::find_by_ids(&mut conn, &ids)
            .await
            .context(QuerySnafu)?;

        let mut grouped: HashMap<TagById, Self::Value> = HashMap::new();
        for value in values.into_iter() {
            let id = TagById::new(&value.id);

            grouped.insert(id, value.into());
        }

        Ok(grouped)
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct TagById(String);

impl TagById {
    pub fn new(id: &str) -> Self {
        Self(String::from(id))
    }
}

impl Hash for TagById {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Loader<TagByPhotoId> for AppLoader {
    type Value = Vec<GqlTag>;
    type Error = Arc<Error>;

    async fn load(
        &self,
        ids: &[TagByPhotoId],
    ) -> Result<HashMap<TagByPhotoId, Self::Value>, Self::Error> {
        let ids: Vec<String> = ids.iter().map(|i| i.0.clone()).collect();

        let mut conn = self.pool.acquire().await.context(ConnectionSnafu)?;

        let values = Tag::find_by_photo_ids(&mut conn, &ids)
            .await
            .context(QuerySnafu)?;

        let mut grouped: HashMap<TagByPhotoId, Self::Value> = HashMap::new();

        for (photo_id, tag) in values.into_iter() {
            let id = TagByPhotoId::new(&photo_id);
            let gql_tag: GqlTag = tag.into();

            grouped.insert_or_push(id, gql_tag);
        }

        Ok(grouped)
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct TagByPhotoId(String);

impl TagByPhotoId {
    pub fn new(id: &str) -> Self {
        Self(String::from(id))
    }
}

impl Hash for TagByPhotoId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to query for tags in loader: {}", source))]
    QueryError { source: DbError },

    #[snafu(display("Failed to acquire connection in tag loader: {}", source))]
    Connection { source: SqlxError },
}
