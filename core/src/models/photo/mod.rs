pub mod db;
mod str;

use serde::{Deserialize, Serialize};
use snafu::prelude::*;
use std::path::Path;
use std::str::FromStr;
use str::filetype::Error as FiletypeError;
use strum_macros::Display as EnumDisplay;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize)]
pub struct Photo {
    pub id: String,
    pub title: String,
    // pub src: String,
    pub filename: String,
    pub filetype: FileType,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted: bool,
}

impl Photo {
    pub fn new(title: String, path: &Path) -> Result<Photo, Error> {
        let id = Uuid::new_v4().to_string();

        println!("PATH {:?}", path);

        let ext = path.extension().context(ExtensionSnafu)?;
        let filetype = FileType::from_str(ext.to_str().unwrap()).context(FiletypeSnafu)?;
        let filename = path.file_name().context(FilenameSnafu)?.to_str().unwrap();

        let now = OffsetDateTime::now_utc().unix_timestamp();
        let created_at = OffsetDateTime::from_unix_timestamp(now).unwrap();
        let updated_at = OffsetDateTime::from_unix_timestamp(now).unwrap();

        Ok(Photo {
            id,
            title: title.to_string(),
            filetype,
            filename: filename.to_string(),
            created_at,
            updated_at,
            deleted: false,
        })
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, EnumDisplay, sqlx::Type, Eq, PartialEq)]
pub enum FileType {
    #[strum(serialize = "JPEG")]
    Jpeg,
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed read extension"))]
    Extension,

    #[snafu(display("Failed to read filename"))]
    Filename,

    #[snafu(display("Invalid FileType: {:?}", source))]
    Filetype { source: FiletypeError },
}
