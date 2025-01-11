use crate::prefetch::PrefetchedCollection;

#[derive(Debug)]
pub struct AppState {
    pub api_host: String,
    pub prefetched: PrefetchedCollection,
}
