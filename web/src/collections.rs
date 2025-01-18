use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Display, PartialEq, EnumString, serde::Serialize, Hash, Eq)]
pub enum Collection {
    #[strum(serialize = "portfolio")]
    #[serde(rename(serialize = "portfolio"))]
    Portfolio,
    #[strum(serialize = "berlin")]
    #[serde(rename(serialize = "berlin"))]
    Berlin,
    #[strum(serialize = "japan")]
    #[serde(rename(serialize = "japan"))]
    Japan,
}

pub static COLLECTIONS: &[Collection] =
    &[Collection::Portfolio, Collection::Berlin, Collection::Japan];
