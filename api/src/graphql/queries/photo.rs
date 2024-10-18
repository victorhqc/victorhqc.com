use crate::graphql::{context::get_conn, models::Photo as GqlPhoto};
use async_graphql::{Context, Object, Result, ID};
use core_victorhqc_com::models::photo::Photo;

#[derive(Default)]
pub struct PhotoQuery;

#[Object]
impl PhotoQuery {
    pub async fn photo(&self, ctx: &Context<'_>, id: ID) -> Result<GqlPhoto> {
        let mut conn = get_conn(ctx).await?;
        let photo = Photo::find_by_id(&mut conn, &id).await?;

        Ok(photo.into())
    }

    pub async fn photos(&self, ctx: &Context<'_>) -> Result<Vec<GqlPhoto>> {
        let mut conn = get_conn(ctx).await?;
        let photos = Photo::find_all(&mut conn).await?;
        let photos = photos.into_iter().map(|p| p.into()).collect();

        Ok(photos)
    }
}
