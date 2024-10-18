use crate::graphql::{context::get_conn, models::ExifMeta as GqlExifMeta};
use async_graphql::{Context, Object, Result, ID};
use core_victorhqc_com::models::exif_meta::ExifMeta;

#[derive(Default)]
pub struct ExifMetaQuery;

#[Object]
impl ExifMetaQuery {
    pub async fn exif_meta(&self, ctx: &Context<'_>, id: ID) -> Result<GqlExifMeta> {
        let mut conn = get_conn(ctx).await?;
        let value = ExifMeta::find_by_id(&mut conn, &id).await?;

        Ok(value.into())
    }
}
