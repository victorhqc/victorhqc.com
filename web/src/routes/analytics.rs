use crate::{collections::Collection, AppState};
use actix_web::{get, http::header, web, HttpRequest, HttpResponse, Responder, Result};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr};
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

    println!("Client ID from header {:?}", client_id);

    client_id
}

#[derive(Debug, Deserialize)]
struct Info {
    #[serde(rename(deserialize = "p"))]
    pub path: String,
}

#[derive(Debug, Serialize, Hash, Eq, PartialEq)]
pub enum Route {
    Index,
    Photography,
    Collection(Collection),
    Photo(String),
}

impl FromStr for Route {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split("/").collect();

        if parts.len() > 2 {
            if let (Some(part), Some(id)) = (parts.get(1), parts.get(2)) {
                return match *part {
                    "collection" => {
                        if let Ok(c) = Collection::from_str(id) {
                            Ok(Route::Collection(c))
                        } else {
                            Err(format!("Invalid route: {}", s))
                        }
                    }
                    "photo" => Ok(Route::Photo(id.to_string())),
                    _ => Err(format!("Invalid route: {}", s)),
                };
            }
        }

        println!("parts {:?}", parts);

        match s {
            "/" => Ok(Route::Index),
            "/photography" => Ok(Route::Photography),
            _ => Err(format!("Invalid route: {}", s)),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Visits {
    routes: HashMap<Route, u64>,
}

impl Visits {
    pub fn new() -> Self {
        Visits {
            routes: HashMap::new(),
        }
    }

    pub fn increment(&mut self, route: Route) {
        if let Some(v) = self.routes.get_mut(&route) {
            *v += 1;
        } else {
            self.routes.insert(route, 1);
        }
    }
}

#[get("/analytics")]
pub async fn init_analytics(
    data: web::Data<AppState>,
    info: web::Query<Info>,
    req: HttpRequest,
) -> Result<impl Responder> {
    println!("Info {:?}", info);
    let route = Route::from_str(&info.path).ok();

    if route.is_none() {
        return Ok(HttpResponse::BadRequest().finish());
    }
    let route = route.unwrap();

    let mut visits = data.visits.lock().unwrap();
    visits.increment(route);

    println!("{:?}", visits);

    let client_id = get_client_id(&req);
    let mut session_map = data.unique_sessions.lock().unwrap();

    if let Some(client_id) = client_id {
        session_map.get(&client_id).cloned().unwrap_or_else(|| {
            let new_session_id = generate_unique_session_id();
            session_map.insert(client_id.clone());

            new_session_id
        });

        return Ok(HttpResponse::Created().finish());
    }

    Ok(HttpResponse::BadRequest().finish())
}
