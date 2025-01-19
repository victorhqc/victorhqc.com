use actix_web::{http::header, HttpRequest, Result};
use log::debug;
use serde::Serialize;
use std::str::FromStr;

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct UniqueId(pub String);

impl FromStr for UniqueId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
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

pub fn get_referer(req: &HttpRequest) -> Option<String> {
    req.headers()
        .get(header::REFERER)
        .and_then(|v| v.to_str().ok())
        .map(String::from)
}
