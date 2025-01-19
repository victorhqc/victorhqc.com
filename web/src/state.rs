use crate::analytics::{session::Session, visit::Visit};
use crate::prefetch::PrefetchedCollection;
use tokio::sync::mpsc::Sender;
use uaparser::UserAgentParser;

#[derive(Debug, Clone)]
pub struct AppState {
    pub api_host: String,
    pub prefetched: PrefetchedCollection,
    pub ua_parser: UserAgentParser,
    pub analytics_sender: Sender<(Session, Visit)>,
}
