use crate::{
    collections::{Collection, COLLECTIONS},
    gql::get_portfolio::GetPortfolioPhotos,
    requests,
};
use snafu::prelude::*;
use std::collections::HashMap;

pub type PrefetchedCollection = HashMap<Collection, Vec<GetPortfolioPhotos>>;

/**
Since we need to fetch the pictures to display them in the fron-end, but also to know which image
to go when navigating with the keyboard. We need to cache the GQL calls to avoid overhead.
*/
pub async fn fetch_photos() -> Result<PrefetchedCollection, Error> {
    let mut prefetched: PrefetchedCollection = HashMap::new();

    for collection in COLLECTIONS {
        let photos = requests::photos::get_photos_from_tag(collection.to_string().as_str())
            .await
            .context(FetchSnafu {
                collection: collection.clone(),
            })?;

        prefetched.insert(collection.clone(), photos);
    }

    Ok(prefetched)
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to fetch photos from {}: {:?}", collection, source))]
    Fetch {
        source: requests::photos::Error,
        collection: Collection,
    },
}
