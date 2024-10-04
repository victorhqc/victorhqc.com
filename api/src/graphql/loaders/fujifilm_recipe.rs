use crate::{graphql::loaders::AppLoader, graphql::models::FujifilmRecipe as GqlFujifilmRecipe};
use async_graphql::{dataloader::Loader, Result};
use core_victorhqc_com::models::fujifilm::{db::Error as DbError, FujifilmRecipe};
use snafu::prelude::*;
use std::{
    cmp::{Eq, PartialEq},
    collections::HashMap,
    hash::{Hash, Hasher},
    sync::Arc,
};

impl Loader<FujifilmRecipeByExifMetaId> for AppLoader {
    type Value = GqlFujifilmRecipe;
    type Error = Arc<Error>;

    async fn load(
        &self,
        ids: &[FujifilmRecipeByExifMetaId],
    ) -> Result<HashMap<FujifilmRecipeByExifMetaId, Self::Value>, Self::Error> {
        let ids: Vec<String> = ids.iter().map(|i| i.0.clone()).collect();

        debug!("IDs {:?}", ids);

        let values = FujifilmRecipe::find_by_exif_meta_ids(&self.pool, &ids)
            .await
            .context(QuerySnafu)?;

        let mut grouped: HashMap<FujifilmRecipeByExifMetaId, Self::Value> = HashMap::new();

        for (exif_meta_id, recipe) in values.into_iter() {
            let id = FujifilmRecipeByExifMetaId::new(&exif_meta_id);
            grouped.insert(id, recipe.into());
        }

        Ok(grouped)
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct FujifilmRecipeByExifMetaId(String);

impl FujifilmRecipeByExifMetaId {
    pub fn new(id: &str) -> Self {
        Self(String::from(id))
    }
}

impl Hash for FujifilmRecipeByExifMetaId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("{:?}", source))]
    QueryError { source: DbError },
}
