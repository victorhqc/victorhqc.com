pub mod db;

use serde::{Deserialize, Serialize};
use strum_macros::{Display as EnumDisplay, EnumString};
use time::OffsetDateTime;

#[derive(Clone, Debug, Serialize)]
pub struct Photo {
    pub id: String,
    pub title: String,
    pub src: String,
    pub filename: String,
    pub filetype: FileType,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted: bool,
}

#[derive(
    Clone, Copy, Debug, Deserialize, Serialize, EnumString, EnumDisplay, sqlx::Type, Eq, PartialEq,
)]
pub enum FileType {
    #[strum(serialize = "JPEG")]
    Jpeg,
}
