use super::record::{generate_unique_session_id, get_client_id};
use crate::collections::Collection;
use crate::state::AppState;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder, Result};
use log::{debug, trace};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;
use std::string::ToString;

#[get("/analytics")]
pub async fn register_visit(
    data: web::Data<AppState>,
    info: web::Query<Info>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let route = Route::from_str(&info.path).ok();

    if route.is_none() {
        debug!("No route given, no analytics to register");

        return Ok(HttpResponse::BadRequest().finish());
    }
    let route = route.unwrap();
    debug!("Registering visit: {:?}", route);

    let mut visits = data.visits.lock().unwrap();
    visits.increment(route);

    trace!("Total Visits: {:?}", visits);

    let client_id = get_client_id(&req);
    let mut session_map = data.unique_sessions.lock().unwrap();

    if let Some(client_id) = client_id {
        session_map.get(&client_id).cloned().unwrap_or_else(|| {
            let new_session_id = generate_unique_session_id();
            session_map.insert(client_id.clone());

            new_session_id
        });

        trace!("Total Sessions: {:?}", session_map);

        return Ok(HttpResponse::Created().finish());
    }

    Ok(HttpResponse::BadRequest().finish())
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

        match s {
            "/" => Ok(Route::Index),
            "/photography" => Ok(Route::Photography),
            _ => Err(format!("Invalid route: {}", s)),
        }
    }
}

impl Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Route::Index => write!(f, "/"),
            Route::Photography => write!(f, "/photography"),
            Route::Collection(c) => write!(f, "/collection/{}", c),
            Route::Photo(p) => write!(f, "/photo/{}", p),
        }
    }
}
