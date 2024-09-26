use super::{ExifMeta, Tag};
use crate::graphql::loaders::exif_meta::ExifMetaId;
use crate::graphql::loaders::tag::PhotoTagId;
use crate::graphql::loaders::AppLoader;
use crate::models::{photo::Photo as PhotoModel, FileType};
use async_graphql::dataloader::DataLoader;
use async_graphql::{ComplexObject, Context, Result, SimpleObject, ID};
use time::format_description;

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
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

    #[graphql(skip)]
    exif_meta_id: String,
}

#[ComplexObject]
impl Photo {
    async fn exif_meta(&self, ctx: &Context<'_>) -> Result<ExifMeta> {
        let loader = ctx.data_unchecked::<DataLoader<AppLoader>>();
        let id = ExifMetaId::new(&self.exif_meta_id);
        let exif_meta: ExifMeta = loader
            .load_one(id)
            .await?
            .expect("Photo has no such Exif Metadata");

        Ok(exif_meta)
    }

    async fn tags(&self, ctx: &Context<'_>) -> Result<Vec<Tag>> {
        let loader = ctx.data_unchecked::<DataLoader<AppLoader>>();
        let id = PhotoTagId::new(&self.id);

        let tags = (loader.load_one(id).await?).unwrap_or_default();

        Ok(tags)
    }
}

impl From<PhotoModel> for Photo {
    fn from(photo: PhotoModel) -> Self {
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
            exif_meta_id: photo.exif_meta_id,
        }
    }
}
