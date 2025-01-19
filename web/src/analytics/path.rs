use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Path {
    pub name: String,
}
