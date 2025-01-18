use actix_web::{http::header, HttpRequest, Result};
use log::debug;
use serde::Serialize;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct UniqueId(String);

impl FromStr for UniqueId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl UniqueId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

pub fn generate_unique_session_id() -> UniqueId {
    UniqueId::new()
}

pub fn get_client_id(req: &HttpRequest) -> Option<UniqueId> {
    let client_id = req
        .headers()
        .get(header::HeaderName::from_static("x-visitor-id"))
        .and_then(|v| v.to_str().ok())
        .and_then(|v| UniqueId::from_str(v).ok());

    debug!("Client ID from header {:?}", client_id);

    client_id
}
