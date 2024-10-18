use crate::graphql::{context::get_conn, models::Tag as GqlTag};
use async_graphql::{Context, Object, Result};
use core_victorhqc_com::models::tag::Tag;

#[derive(Default)]
pub struct TagQuery;

#[Object]
impl TagQuery {
    async fn tag(&self, ctx: &Context<'_>, name: String) -> Result<GqlTag> {
        let mut conn = get_conn(ctx).await?;
        let tag = Tag::find_by_name(&mut conn, &name).await?;

        Ok(tag.into())
    }
}
