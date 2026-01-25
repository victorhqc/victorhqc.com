use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Display, PartialEq, EnumString, serde::Serialize, Hash, Eq)]
pub enum Collection {
    #[strum(serialize = "portfolio")]
    #[serde(rename(serialize = "portfolio"))]
    Portfolio,
    #[strum(serialize = "street")]
    #[serde(rename(serialize = "street"))]
    Street,
    #[strum(serialize = "postcards")]
    #[serde(rename(serialize = "postcards"))]
    Postcards,
    #[strum(serialize = "film")]
    #[serde(rename(serialize = "film"))]
    Film,
    #[strum(serialize = "berlin")]
    #[serde(rename(serialize = "berlin"))]
    Berlin,
    #[strum(serialize = "japan")]
    #[serde(rename(serialize = "japan"))]
    Japan,
    #[strum(serialize = "mexico")]
    #[serde(rename(serialize = "mexico"))]
    Mexico,
    #[strum(serialize = "bikes")]
    #[serde(rename(serialize = "bikes"))]
    Bikes,
    #[strum(serialize = "close-ups")]
    #[serde(rename(serialize = "close-ups"))]
    CloseUps,
}

pub static COLLECTIONS: &[Collection] = &[
    Collection::Portfolio,
    Collection::Street,
    Collection::Postcards,
    Collection::Film,
    Collection::Mexico,
    Collection::Berlin,
    Collection::Japan,
    Collection::Bikes,
    Collection::CloseUps,
];
