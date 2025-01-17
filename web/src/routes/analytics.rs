use actix_web::{http::header, HttpRequest};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UniqueId(String);

#[derive(Debug)]
pub struct UserAgent(String);

impl UserAgent {
    pub fn get(&self) -> &str {
        &self.0
    }
}

pub fn generate_unique_etag(client_id: &UniqueId) -> String {
    format!("{}__{}", client_id.0, Uuid::new_v4())
}

pub fn generate_unique_session_id() -> UniqueId {
    UniqueId(Uuid::new_v4().to_string())
}

pub fn get_client_id(req: &HttpRequest) -> (UniqueId, UserAgent) {
    let ip = req
        .peer_addr()
        .map(|addr| addr.ip().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    let user_agent = req
        .headers()
        .get(header::USER_AGENT)
        .and_then(|ua| ua.to_str().ok())
        .unwrap_or("unknown");

    (
        UniqueId(format!("{}__{}", ip, Uuid::new_v4())),
        UserAgent(user_agent.to_string()),
    )
}
