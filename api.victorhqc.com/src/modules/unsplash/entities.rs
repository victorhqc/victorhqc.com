use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Picture {
    pub id: String,
    pub width: u32,
    pub height: u32,
    pub color: Option<String>,
    pub blur_hash: Option<String>,
    pub downloads: u64,
    pub likes: u64,
    pub description: Option<String>,
    pub alt_description: Option<String>,
    pub location: Location,
    pub exif: Exif,
    pub links: Links,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Links {
    #[serde(rename = "self")]
    pub l_self: String,
    pub html: String,
    pub download: String,
    pub download_location: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub name: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Exif {
    pub make: Option<String>,
    pub model: Option<String>,
    pub exposure_time: Option<String>,
    pub aperture: Option<String>,
    pub focal_length: Option<String>,
    pub iso: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub name: String,
    pub portfolio_url: Option<String>,
    pub bio: Option<String>,
    pub location: Option<String>,
    pub instagram_username: Option<String>,
    pub twitter_username: Option<String>,
    pub links: UserLinks,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLinks {
    #[serde(rename = "self")]
    pub l_self: String,
    pub html: String,
    pub photos: String,
    pub likes: String,
    pub portfolio: String,
}
