use std::num::{ParseFloatError, ParseIntError};
use once_cell::sync::Lazy;
use regex::Regex;
use snafu::prelude::*;

pub mod json;

pub type Tag = String;
pub type Value = String;

#[derive(Debug, Clone)]
pub struct ExifData(Tag,Value);

impl ExifData {
    pub fn tag(&self) -> &str {
        &self.0
    }

    pub fn value(&self) -> &str {
        &self.1
    }
}

pub trait FromExifData {
    fn from_exif(data: &[ExifData]) -> Option<Self>
    where
        Self: Sized;
}

pub trait FindExifData {
    fn find(&self, exif_tag: &str) -> Option<ExifData>;
}

impl FindExifData for &[ExifData] {
    fn find(&self, exif_tag: &str) -> Option<ExifData> {
        let found = self.iter().find(|exif| exif.tag() == exif_tag);

        if let Some(exif) = found {
            let exif = exif.clone();

            return Some(exif);
        }

        None
    }
}

impl TryFrom<ExifData> for i64 {
    type Error = Error;

    fn try_from(exif: ExifData) -> Result<i64, Error> {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i:([+\-0-9]+))").unwrap());

        let captures = RE.captures(exif.value()).context(NotFoundSnafu)?;

        let value = if let Some(v) = captures.get(1) {
            String::from(v.as_str())
        } else {
            return Err(Error::NotFound);
        };

        let value = value.parse::<i64>().context(ParseIntSnafu)?;

        Ok(value)
    }
}

impl TryFrom<ExifData> for f64 {
    type Error = Error;

    fn try_from(exif: ExifData) -> Result<f64, Error> {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i:([+\-0-9.]+))").unwrap());

        let captures = RE.captures(exif.value()).context(NotFoundSnafu)?;

        let value = if let Some(v) = captures.get(1) {
            String::from(v.as_str())
        } else {
            return Err(Error::NotFound);
        };

        let value = value.parse::<f64>().context(ParseFloatSnafu)?;

        Ok(value)
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Could not find proper value"))]
    NotFound,

    #[snafu(display("Failed to parse i64: {:?}", source))]
    ParseInt { source: ParseIntError },

    #[snafu(display("Failed to parse f64: {:?}", source))]
    ParseFloat { source: ParseFloatError },
}
