use crate::{graphql::loaders::AppLoader, graphql::models::Photo as GqlPhoto};
use async_graphql::{dataloader::Loader, Result};
use core_victorhqc_com::{
    models::photo::{db::Error as DbError, Photo},
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

impl Loader<PhotoByTagId> for AppLoader {
    type Value = Vec<GqlPhoto>;
    type Error = Arc<Error>;

    async fn load(
        &self,
        ids: &[PhotoByTagId],
    ) -> Result<HashMap<PhotoByTagId, Self::Value>, Self::Error> {
        let ids: Vec<String> = ids.iter().map(|i| i.0.clone()).collect();

        let mut conn = self.pool.acquire().await.context(ConnectionSnafu)?;

        let values = Photo::find_by_tag_ids(&mut conn, &ids)
            .await
            .context(QuerySnafu)?;

        let mut grouped: HashMap<PhotoByTagId, Self::Value> = HashMap::new();

        for (tag_id, photo) in values.into_iter() {
            let id = PhotoByTagId::new(&tag_id);
            let gql: GqlPhoto = photo.into();

            grouped.insert_or_push(id, gql);
        }

        Ok(grouped)
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct PhotoByTagId(String);

impl PhotoByTagId {
    pub fn new(id: &str) -> Self {
        Self(String::from(id))
    }
}

impl Hash for PhotoByTagId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to query for photos in loader: {}", source))]
    QueryError { source: DbError },

    #[snafu(display("Failed to acquire connection in photo loader: {}", source))]
    Connection { source: SqlxError },
}
