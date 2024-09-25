use crate::graphql::context::get_pool;
use crate::graphql::models::Photo as GqlPhoto;
use crate::models::photo::Photo;
use async_graphql::{Context, Object, Result, ID};

#[derive(Default)]
pub struct PhotoQuery;

#[Object]
impl PhotoQuery {
    pub async fn photo(&self, ctx: &Context<'_>, id: ID) -> Result<GqlPhoto> {
        let pool = get_pool(ctx).await?;
        let photo = Photo::find_by_id(pool, &id).await?;

        Ok(photo.into())
    }
}
