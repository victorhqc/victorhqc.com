use super::{ExifMeta, Tag};
use crate::graphql::loaders::{exif_meta::ExifMetaByPhotoId, tag::TagByPhotoId, AppLoader};
use async_graphql::{
    dataloader::DataLoader, ComplexObject, Context, Enum, Result, SimpleObject, ID,
};
use core_victorhqc_com::models::photo::{FileType as CoreFileType, Photo as CorePhoto};

#[derive(Enum, Copy, Clone, Debug, Eq, PartialEq)]
pub enum FileType {
    Jpeg,
}

impl From<CoreFileType> for FileType {
    fn from(value: CoreFileType) -> Self {
        match value {
            CoreFileType::Jpeg => FileType::Jpeg,
        }
    }
}

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct Photo {
    pub id: ID,
    pub title: String,
    pub src: String,
    pub filename: String,
    pub filetype: FileType,
    pub created_at: String,
    pub updated_at: String,
    pub deleted: bool,
}

#[ComplexObject]
impl Photo {
    async fn exif_meta(&self, ctx: &Context<'_>) -> Result<ExifMeta> {
        let loader = ctx.data_unchecked::<DataLoader<AppLoader>>();
        let id = ExifMetaByPhotoId::new(&self.id);
        let exif_meta: ExifMeta = loader
            .load_one(id)
            .await?
            .expect("Photo has no such Exif Metadata");

        Ok(exif_meta)
    }

    async fn tags(&self, ctx: &Context<'_>) -> Result<Vec<Tag>> {
        let loader = ctx.data_unchecked::<DataLoader<AppLoader>>();
        let id = TagByPhotoId::new(&self.id);

        let tags = loader.load_one(id).await?.unwrap_or_default();

        Ok(tags)
    }
}

impl From<CorePhoto> for Photo {
    fn from(photo: CorePhoto) -> Self {
        Photo {
            id: photo.id.into(),
            title: photo.title,
            src: "TODO".to_string(),
            filename: photo.filename,
            filetype: photo.filetype.into(),
            created_at: format!("{}", photo.created_at),
            updated_at: format!("{}", photo.updated_at),
            deleted: photo.deleted,
        }
    }
}
