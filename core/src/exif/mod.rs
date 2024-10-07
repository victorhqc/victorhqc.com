use once_cell::sync::Lazy;
use regex::Regex;

pub mod json;

pub type Tag = String;
pub type Value = String;

#[derive(Debug, Clone)]
pub struct ExifData(pub Tag, pub Value);

pub trait FromExifData {
    fn from_exif(data: &Vec<ExifData>) -> Option<Self>;
}

pub trait FindExifData {
    fn find(&self, exif_tag: &str) -> Option<ExifData>;
}

impl FindExifData for Vec<ExifData> {
    fn find(&self, exif_tag: &str) -> Option<ExifData> {
        let found = self.iter().find(|exif| exif.0 == exif_tag);

        if let Some(exif) = found {
            let exif = exif.clone();

            return Some(exif);
        }

        None
    }
}

impl From<ExifData> for Option<i64> {
    fn from(exif: ExifData) -> Option<i64> {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i:([+\-0-9]+))").unwrap());

        let captures = RE.captures(&exif.1)?;

        let value = if let Some(v) = captures.get(1) {
            String::from(v.as_str())
        } else {
            return None;
        };

        if let Ok(v) = value.parse::<i64>() {
            Some(v)
        } else {
            None
        }
    }
}

impl From<ExifData> for Option<f64> {
    fn from(exif: ExifData) -> Option<f64> {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i:([+\-0-9.]+))").unwrap());

        let captures = RE.captures(&exif.1)?;

        let value = if let Some(v) = captures.get(1) {
            String::from(v.as_str())
        } else {
            return None;
        };

        if let Ok(v) = value.parse::<f64>() {
            Some(v)
        } else {
            None
        }
    }
}
