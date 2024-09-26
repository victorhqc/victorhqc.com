use crate::graphql::loaders::photo::TagPhotoId;
use crate::graphql::loaders::AppLoader;
use crate::graphql::models::Photo;
use crate::models::tag::Tag as TagModel;
use async_graphql::dataloader::DataLoader;
use async_graphql::{ComplexObject, Context, Result, SimpleObject, ID};

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct Tag {
    pub id: ID,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted: bool,
}

#[ComplexObject]
impl Tag {
    async fn photos(&self, ctx: &Context<'_>) -> Result<Vec<Photo>> {
        let loader = ctx.data_unchecked::<DataLoader<AppLoader>>();
        let id = TagPhotoId::new(&self.id);

        let photos = (loader.load_one(id).await?).unwrap_or_default();

        Ok(photos)
    }
}

impl From<TagModel> for Tag {
    fn from(value: TagModel) -> Self {
        Tag {
            id: value.id.into(),
            name: value.name,
            created_at: format!("{}", value.created_at),
            updated_at: format!("{}", value.updated_at),
            deleted: value.deleted,
        }
    }
}
