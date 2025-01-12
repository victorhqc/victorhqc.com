use crate::prefetch::PrefetchedCollection;
use uaparser::UserAgentParser;

#[derive(Debug)]
pub struct AppState {
    pub api_host: String,
    pub prefetched: PrefetchedCollection,
    pub ua_parser: UserAgentParser,
}
