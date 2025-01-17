use crate::prefetch::PrefetchedCollection;
use crate::routes::analytics::{UniqueId, Visits};
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};
use uaparser::UserAgentParser;

#[derive(Debug, Clone)]
pub struct AppState {
    pub api_host: String,
    pub prefetched: PrefetchedCollection,
    pub ua_parser: UserAgentParser,
    // pub visitor_etags: Arc<Mutex<HashMap<UniqueId, String>>>,
    pub unique_sessions: Arc<Mutex<HashSet<UniqueId>>>,
    pub visits: Arc<Mutex<Visits>>,
}
