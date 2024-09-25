use crate::db::photos::get_photo_by_id;
use crate::graphql::context::get_pool;
use crate::graphql::models::Photo;
use async_graphql::{Context, Object, Result, ID};

#[derive(Default)]
pub struct PhotoQuery;

#[Object]
impl PhotoQuery {
    pub async fn photo(&self, ctx: &Context<'_>, id: ID) -> Result<Photo> {
        let pool = get_pool(ctx).await?;
        let photo = get_photo_by_id(pool, &id).await?;

        Ok(photo.into())
    }
}
