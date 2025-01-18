use actix_web::{http::header, HttpRequest};

#[derive(Debug)]
pub struct UserAgent(String);

impl UserAgent {
    pub fn get(&self) -> &str {
        &self.0
    }
}

pub fn get_user_agent(req: &HttpRequest) -> UserAgent {
    let user_agent = req
        .headers()
        .get(header::USER_AGENT)
        .and_then(|ua| ua.to_str().ok())
        .unwrap_or("unknown");

    UserAgent(user_agent.to_string())
}
