use crate::models::{FileType, Photo as AppPhoto};
use async_graphql::{SimpleObject, ID};
use time::format_description;

#[derive(SimpleObject, Clone)]
pub struct Photo {
    pub id: ID,
    pub src: String,
    pub filename: String,
    pub rating: i8,
    pub filetype: FileType,
    pub date_taken: Option<String>,
    pub city: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub deleted: bool,
}

impl From<AppPhoto> for Photo {
    fn from(photo: AppPhoto) -> Self {
        let date_taken = if let Some(d) = photo.date_taken {
            let format = format_description::parse("[year]-[month]-[day]").unwrap();
            let formatted = d.format(&format).unwrap();
            Some(formatted)
        } else {
            None
        };

        Photo {
            id: photo.id.into(),
            src: photo.src,
            filename: photo.filename,
            rating: photo.rating,
            filetype: photo.filetype,
            date_taken,
            city: photo.city,
            created_at: format!("{}", photo.created_at),
            updated_at: format!("{}", photo.updated_at),
            deleted: photo.deleted,
        }
    }
}
