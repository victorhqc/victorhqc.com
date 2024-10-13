use crate::models::photo::FileType;
use snafu::prelude::*;
use std::str::FromStr;

impl FromStr for FileType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "jpg" => Ok(FileType::Jpeg),
            "jpeg" => Ok(FileType::Jpeg),
            _ => Err(Error::Invalid { ext: s.to_string() }),
        }
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Invalid Extension: {}", ext))]
    Invalid { ext: String },
}
