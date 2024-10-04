use crate::graphql::{context::get_pool, models::ExifMeta as GqlExifMeta};
use async_graphql::{Context, Object, Result, ID};
use core_victorhqc_com::models::exif_meta::ExifMeta;

#[derive(Default)]
pub struct ExifMetaQuery;

#[Object]
impl ExifMetaQuery {
    pub async fn exif_meta(&self, ctx: &Context<'_>, id: ID) -> Result<GqlExifMeta> {
        let pool = get_pool(ctx).await?;
        let value = ExifMeta::find_by_id(pool, &id).await?;

        Ok(value.into())
    }
}
