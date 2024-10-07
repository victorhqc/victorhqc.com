pub mod json;

pub type Tag = String;
pub type Value = String;

#[derive(Debug, Clone)]
pub struct ExifData(pub Tag, pub Value);
