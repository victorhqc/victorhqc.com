use crate::graphql::context::get_pool;
use crate::graphql::models::Tag as GqlTag;
use crate::models::tag::Tag;
use async_graphql::{Context, Object, Result};

#[derive(Default)]
pub struct TagQuery;

#[Object]
impl TagQuery {
    async fn tag(&self, ctx: &Context<'_>, name: String) -> Result<GqlTag> {
        let pool = get_pool(ctx).await?;
        let tag = Tag::find_by_name(pool, &name).await?;

        Ok(tag.into())
    }
}
