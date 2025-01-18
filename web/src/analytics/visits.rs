use super::routes::Route;
use serde::Serialize;
use std::collections::HashMap;

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
